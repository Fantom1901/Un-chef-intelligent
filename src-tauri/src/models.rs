//! Rust-зеркало TS-типов из `src/lib/types/recipe.ts`.
//! JSON-поля — camelCase, значения enum — lowercase.

// Часть типов пока не используется (появится в Фазах 2–3), но обязана зеркалить TS.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IngredientCategory {
    Vegetables,
    Fruits,
    Meat,
    Fish,
    Dairy,
    Grains,
    Pantry,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub id: String,
    pub name: String,
    pub emoji: String,
    pub category: IngredientCategory,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeRequest {
    pub ingredients: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servings: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Строгий формат ответа LLM. Лишние/недостающие поля — ошибка парсинга.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeResponse {
    pub title: String,
    pub cooking_time_minutes: u32,
    pub used_ingredients: Vec<String>,
    pub missing_ingredients: Vec<String>,
    pub steps: Vec<String>,
    pub chef_tip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedRecipe {
    pub id: i64,
    pub created_at: String,
    pub request: RecipeRequest,
    pub recipe: RecipeResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AppSettings {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.openai.com/v1".into(),
            model: "gpt-4o-mini".into(),
        }
    }
}
