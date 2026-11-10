//! SQLite-схема. Миграции применяются `tauri-plugin-sql` при старте
//! (база предзагружается через `plugins.sql.preload` в tauri.conf.json).
//! Фронтенд работает с БД через `@tauri-apps/plugin-sql`.

use tauri_plugin_sql::{Migration, MigrationKind};

pub const DB_URL: &str = "sqlite:leftover_chef.db";

const INIT_SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS recipes (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at   TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    title        TEXT    NOT NULL,
    request_json TEXT    NOT NULL,
    recipe_json  TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_recipes_created_at ON recipes (created_at DESC);

CREATE TABLE IF NOT EXISTS custom_products (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT NOT NULL UNIQUE COLLATE NOCASE,
    emoji      TEXT NOT NULL DEFAULT '🍽️',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
";

pub fn migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "create_recipes_and_custom_products",
        sql: INIT_SCHEMA,
        kind: MigrationKind::Up,
    }]
}
