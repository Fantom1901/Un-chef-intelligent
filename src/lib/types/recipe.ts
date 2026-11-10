/**
 * Доменные типы приложения. Зеркалятся в `src-tauri/src/models.rs`
 * (serde, `rename_all = "camelCase"`). При изменении правь оба файла.
 */

export type IngredientCategory =
  | 'vegetables'
  | 'fruits'
  | 'meat'
  | 'fish'
  | 'dairy'
  | 'grains'
  | 'pantry'
  | 'other';

/** Продукт в сетке выбора (встроенный или добавленный пользователем). */
export interface Ingredient {
  id: string;
  name: string;
  emoji: string;
  category: IngredientCategory;
  isCustom: boolean;
}

/** Запрос на генерацию рецепта, уходит в Rust-команду `generate_recipe`. */
export interface RecipeRequest {
  /** Названия выбранных продуктов. */
  ingredients: string[];
  servings?: number;
  /** Свободные пожелания: «без глютена», «быстро» и т.п. */
  notes?: string;
}

/** Строго структурированный рецепт, который возвращает LLM. */
export interface RecipeResponse {
  title: string;
  cookingTimeMinutes: number;
  /** Какие из имеющихся продуктов использованы. */
  usedIngredients: string[];
  /** Что нужно докупить. */
  missingIngredients: string[];
  steps: string[];
  chefTip: string;
}

/** Запись в истории (таблица `recipes` в SQLite). */
export interface SavedRecipe {
  id: number;
  createdAt: string;
  request: RecipeRequest;
  recipe: RecipeResponse;
}

/** Настройки LLM-провайдера (tauri-plugin-store, файл `settings.json`). */
export interface AppSettings {
  apiKey: string;
  baseUrl: string;
  model: string;
}

export const DEFAULT_SETTINGS: AppSettings = {
  apiKey: '',
  baseUrl: 'https://api.openai.com/v1',
  model: 'gpt-4o-mini'
};

export type TabId = 'products' | 'history' | 'settings';
