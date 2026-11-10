import { invoke } from '@tauri-apps/api/core';
import type { AppSettings, RecipeRequest, RecipeResponse } from '$lib/types/recipe';

export const api = {
  getSettings: () => invoke<AppSettings>('get_settings'),
  saveSettings: (settings: AppSettings) => invoke<void>('save_settings', { settings }),
  generateRecipe: (request: RecipeRequest) =>
    invoke<RecipeResponse>('generate_recipe', { request })
};
