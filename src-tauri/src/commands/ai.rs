//! Запрос к OpenAI-compatible API и строгая валидация ответа.
//! Каркас фазы 1: полный путь запрос → парсинг → валидация; промпт будет дорабатываться в Фазе 3.

use std::time::Duration;

use serde_json::{json, Value};
use tauri::{AppHandle, Runtime};

use crate::commands::settings;
use crate::models::{RecipeRequest, RecipeResponse};

const SYSTEM_PROMPT: &str = "Ты — опытный шеф-повар. Пользователь называет продукты, которые у него есть. \
Придумай один рецепт, максимально используя эти продукты. \
Отвечай СТРОГО одним JSON-объектом без markdown и пояснений со следующими полями: \
title (string), cookingTimeMinutes (integer), usedIngredients (string[]), \
missingIngredients (string[]), steps (string[]), chefTip (string). \
Пиши на русском языке.";

fn build_user_prompt(request: &RecipeRequest) -> String {
    let mut prompt = format!("Мои продукты: {}.", request.ingredients.join(", "));
    if let Some(servings) = request.servings {
        prompt.push_str(&format!(" Порций: {servings}."));
    }
    if let Some(notes) = request.notes.as_deref().filter(|n| !n.trim().is_empty()) {
        prompt.push_str(&format!(" Пожелания: {notes}."));
    }
    prompt
}

/// Убирает возможные ```json ... ``` обёртки, которые добавляют некоторые модели.
fn strip_code_fence(content: &str) -> &str {
    let trimmed = content.trim();
    let Some(rest) = trimmed.strip_prefix("```") else {
        return trimmed;
    };
    let rest = rest.strip_prefix("json").unwrap_or(rest);
    rest.trim().strip_suffix("```").unwrap_or(rest).trim()
}

fn validate(recipe: &RecipeResponse) -> Result<(), String> {
    if recipe.title.trim().is_empty() {
        return Err("В ответе модели пустое название рецепта".into());
    }
    if recipe.steps.is_empty() {
        return Err("В ответе модели нет шагов приготовления".into());
    }
    if recipe.cooking_time_minutes == 0 {
        return Err("В ответе модели некорректное время приготовления".into());
    }
    Ok(())
}

#[tauri::command]
pub async fn generate_recipe<R: Runtime>(
    app: AppHandle<R>,
    request: RecipeRequest,
) -> Result<RecipeResponse, String> {
    if request.ingredients.is_empty() {
        return Err("Выберите хотя бы один продукт".into());
    }

    let settings = settings::load(&app)?;
    if settings.api_key.trim().is_empty() {
        return Err("Не задан API-ключ. Укажите его в настройках".into());
    }

    let url = format!("{}/chat/completions", settings.base_url.trim_end_matches('/'));
    let body = json!({
        "model": settings.model,
        "temperature": 0.7,
        "response_format": { "type": "json_object" },
        "messages": [
            { "role": "system", "content": SYSTEM_PROMPT },
            { "role": "user", "content": build_user_prompt(&request) }
        ]
    });

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(90))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .post(&url)
        .bearer_auth(settings.api_key.trim())
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Сетевая ошибка: {e}"))?;

    let status = response.status();
    if !status.is_success() {
        let details = response.text().await.unwrap_or_default();
        return Err(format!("API вернул {status}: {details}"));
    }

    let payload: Value = response
        .json()
        .await
        .map_err(|e| format!("Некорректный ответ API: {e}"))?;

    let content = payload["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("В ответе API нет choices[0].message.content")?;

    let recipe: RecipeResponse = serde_json::from_str(strip_code_fence(content))
        .map_err(|e| format!("Ответ модели не соответствует схеме рецепта: {e}"))?;
    validate(&recipe)?;

    Ok(recipe)
}
