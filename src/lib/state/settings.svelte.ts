import { api } from '$lib/api/tauri';
import { DEFAULT_SETTINGS, type AppSettings } from '$lib/types/recipe';

class SettingsState {
  apiKey = $state(DEFAULT_SETTINGS.apiKey);
  baseUrl = $state(DEFAULT_SETTINGS.baseUrl);
  model = $state(DEFAULT_SETTINGS.model);

  loaded = $state(false);
  saving = $state(false);
  error = $state<string | null>(null);

  isConfigured = $derived(this.apiKey.trim() !== '' && this.baseUrl.trim() !== '' && this.model.trim() !== '');

  snapshot(): AppSettings {
    return { apiKey: this.apiKey, baseUrl: this.baseUrl, model: this.model };
  }

  async load() {
    try {
      const s = await api.getSettings();
      this.apiKey = s.apiKey;
      this.baseUrl = s.baseUrl;
      this.model = s.model;
      this.error = null;
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loaded = true;
    }
  }

  async save() {
    this.saving = true;
    try {
      await api.saveSettings(this.snapshot());
      this.error = null;
    } catch (e) {
      this.error = String(e);
    } finally {
      this.saving = false;
    }
  }
}

export const settings = new SettingsState();
