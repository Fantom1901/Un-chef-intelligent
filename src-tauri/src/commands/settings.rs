//! Настройки (API-ключ, Base URL, модель) в `tauri-plugin-store`.

use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

use crate::models::AppSettings;

const STORE_FILE: &str = "settings.json";
const SETTINGS_KEY: &str = "settings";

pub fn load<R: Runtime>(app: &AppHandle<R>) -> Result<AppSettings, String> {
    let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
    match store.get(SETTINGS_KEY) {
        Some(value) => serde_json::from_value(value).map_err(|e| e.to_string()),
        None => Ok(AppSettings::default()),
    }
}

#[tauri::command]
pub fn get_settings<R: Runtime>(app: AppHandle<R>) -> Result<AppSettings, String> {
    load(&app)
}

#[tauri::command]
pub fn save_settings<R: Runtime>(app: AppHandle<R>, settings: AppSettings) -> Result<(), String> {
    let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
    let value = serde_json::to_value(&settings).map_err(|e| e.to_string())?;
    store.set(SETTINGS_KEY, value);
    store.save().map_err(|e| e.to_string())
}
