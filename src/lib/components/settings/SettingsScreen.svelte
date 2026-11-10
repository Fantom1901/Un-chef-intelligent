<script lang="ts">
  import Save from 'lucide-svelte/icons/save';
  import ScreenHeader from '$lib/components/ui/ScreenHeader.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { settings } from '$lib/state/settings.svelte';

  const inputClass =
    'border-line bg-surface-raised focus:border-brand-400 h-12 w-full rounded-xl border px-4 text-base outline-none';
</script>

<ScreenHeader title="Настройки" subtitle="Параметры LLM-провайдера хранятся только на устройстве" />

<form
  class="flex flex-col gap-4 px-5 pb-8"
  onsubmit={(e) => {
    e.preventDefault();
    void settings.save();
  }}
>
  <label class="flex flex-col gap-1.5 text-sm font-medium">
    API-ключ
    <input
      type="password"
      bind:value={settings.apiKey}
      autocomplete="off"
      autocapitalize="off"
      spellcheck="false"
      placeholder="sk-…"
      class={inputClass}
    />
  </label>

  <label class="flex flex-col gap-1.5 text-sm font-medium">
    Base URL
    <input
      type="url"
      bind:value={settings.baseUrl}
      autocapitalize="off"
      spellcheck="false"
      placeholder="https://api.openai.com/v1"
      class={inputClass}
    />
  </label>

  <label class="flex flex-col gap-1.5 text-sm font-medium">
    Модель
    <input
      type="text"
      bind:value={settings.model}
      autocapitalize="off"
      spellcheck="false"
      placeholder="gpt-4o-mini"
      class={inputClass}
    />
  </label>

  {#if settings.error}
    <p class="text-sm text-red-600">{settings.error}</p>
  {/if}

  <Button type="submit" disabled={settings.saving}>
    <Save size={20} />
    {settings.saving ? 'Сохранение…' : 'Сохранить'}
  </Button>
</form>
