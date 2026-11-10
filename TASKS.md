# TASKS.md — дорожная карта «Шеф по остаткам»

## Фаза 1 — Scaffolding & Core Architecture 🔄 (выполняется)
- [x] Vite + Svelte 5 (runes) + TypeScript, Tailwind v4 (`@tailwindcss/vite`)
- [x] Tauri v2: `Cargo.toml`, `tauri.conf.json`, `capabilities/default.json`, иконки
- [x] `lib.rs` с `#[cfg_attr(mobile, tauri::mobile_entry_point)]`, плагины sql/store/http
- [x] Модели Rust ⇄ TS-типы, каркасы команд `generate_recipe`, `get_settings`, `save_settings`
- [x] SQLite-схема и миграции (`recipes`, `custom_products`)
- [x] Layout: SafeAreaContainer, BottomNav на 3 вкладки, учёт клавиатуры
- [x] `CLAUDE.md`, `TASKS.md`
- [ ] Проверка на Android: `npm run android:init`, запуск на эмуляторе/устройстве, проверка safe-area и клавиатуры

## Фаза 2 — Продукты и локальные данные
- [ ] Сохранение кастомных продуктов в SQLite (`custom_products`), загрузка при старте
- [ ] Категории/фильтры/поиск в сетке продуктов
- [ ] Удаление кастомных продуктов (long-press)
- [ ] Обёртка над `@tauri-apps/plugin-sql` (`src/lib/api/db.ts`)

## Фаза 3 — Генерация рецептов (LLM)
- [ ] Финальный системный промпт шефа, поля `servings`/`notes` в UI
- [ ] Жёсткая валидация ответа, повтор запроса при невалидном JSON
- [ ] Состояния загрузки/ошибок/отмены, понятные сообщения (401, 429, таймаут)
- [ ] Сохранение рецепта в историю после генерации

## Фаза 4 — История
- [ ] Список рецептов из SQLite, детальный экран (`RecipeCard`)
- [ ] Удаление записей, очистка истории
- [ ] Поиск по истории

## Фаза 5 — Настройки
- [ ] Валидация Base URL, кнопка «Проверить соединение»
- [ ] Показ/скрытие API-ключа, сброс к значениям по умолчанию
- [ ] Выбор модели из списка `/models` (опционально)

## Фаза 6 — Полировка и релиз
- [ ] Иконка/сплэш, тёмная тема
- [ ] Анимации переходов, haptics
- [ ] Подпись APK/AAB, release-сборка, тесты на реальных устройствах
