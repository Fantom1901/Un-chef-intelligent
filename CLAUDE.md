# CLAUDE.md — «Шеф по остаткам» (Leftover Chef)

Offline-first Android-приложение: пользователь выбирает продукты, Rust-бэкенд запрашивает у LLM
(OpenAI-compatible API) рецепт в строгом JSON. История и настройки хранятся только на устройстве.

## Стек (версии не понижать)
- Tauri **v2** (таргет Android) · Rust, `src-tauri/`
- Svelte **5** (TypeScript) + Vite, чистый SPA (без SvelteKit)
- TailwindCSS **v4** через `@tailwindcss/vite`; конфигурация только в `src/app.css` (`@import "tailwindcss"`, `@theme`, `@utility`)
- `bits-ui` (Dialog как Bottom Sheet), `lucide-svelte` (импорт по иконке: `lucide-svelte/icons/<name>`)
- Rust: `reqwest 0.12` (`json`, `rustls-tls`), `serde`, `tauri-plugin-sql` (sqlite), `tauri-plugin-store`, `tauri-plugin-http`

## Жёсткие запреты
- **Svelte 4 запрещён:** никаких `export let`, `$:`, `writable/readable` stores, `on:click` (используй `onclick`), `<slot>` (используй `{#snippet}`/`{@render}`).
  Только руны: `$state`, `$derived`, `$effect`, `$props`, `$bindable`.
- **Tailwind v3 запрещён:** не создавать `tailwind.config.js`, `postcss.config.js`, не использовать `@tailwind base/components/utilities`.
- Сетевые запросы к LLM — только из Rust (`commands/ai.rs`), не из WebView (CORS, валидация ответа).

## Структура
- `src/App.svelte` — каркас: 3 вкладки, SafeArea, прячет BottomNav при открытой клавиатуре
- `src/lib/types/recipe.ts` ⇄ `src-tauri/src/models.rs` — **зеркальные** типы (camelCase в JSON); менять только парой
- `src/lib/state/*.svelte.ts` — глобальное состояние на классах с рунами (`ingredients`, `settings`, `navigation`)
- `src/lib/api/tauri.ts` — единственное место с `invoke(...)`
- `src/lib/components/{layout,products,recipe,history,settings,ui}/`
- `src-tauri/src/commands/{ai,db,settings}.rs`, регистрация плагинов и команд — `src-tauri/src/lib.rs`
- `src-tauri/capabilities/default.json` — права Tauri v2 (sql, store, http)

## Команды
```bash
npm install
npm run dev              # только фронтенд (Vite, :1420)
npm run check            # svelte-check (TS + Svelte)
npm run build            # сборка фронтенда в dist/
cargo check --manifest-path src-tauri/Cargo.toml
npm run tauri dev        # десктопное окно для разработки

# Android (нужны Android SDK/NDK, JDK 17+, ANDROID_HOME, NDK_HOME, rust-таргеты)
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
npm run android:init     # один раз: генерирует src-tauri/gen/android
npm run android:dev
npm run android:build
```

## Соглашения
- Safe area: утилиты `pt-safe`, `pb-safe`, `px-safe`, `h-app` (см. `app.css`); `viewport-fit=cover` в `index.html`.
- Текст по умолчанию не выделяется; для контента рецепта используй `select-allow`.
- Настройки (API-ключ) — только `tauri-plugin-store`, в логи/консоль не выводить.
- Схема SQLite — только через миграции в `commands/db.rs` (новая версия = новый `Migration`, старые не править).
