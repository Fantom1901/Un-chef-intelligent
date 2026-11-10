<script lang="ts">
  import { Dialog } from 'bits-ui';
  import Sparkles from 'lucide-svelte/icons/sparkles';
  import X from 'lucide-svelte/icons/x';
  import Button from '$lib/components/ui/Button.svelte';
  import RecipeCard from './RecipeCard.svelte';
  import { api } from '$lib/api/tauri';
  import { ingredients } from '$lib/state/ingredients.svelte';
  import { settings } from '$lib/state/settings.svelte';
  import { navigation } from '$lib/state/navigation.svelte';
  import type { RecipeResponse } from '$lib/types/recipe';

  let { open = $bindable(false) }: { open?: boolean } = $props();

  let loading = $state(false);
  let error = $state<string | null>(null);
  let recipe = $state<RecipeResponse | null>(null);

  async function generate() {
    loading = true;
    error = null;
    try {
      recipe = await api.generateRecipe({ ingredients: ingredients.selectedNames });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay
      class="data-[state=open]:animate-fade-in data-[state=closed]:animate-fade-out fixed inset-0 z-40 bg-black/40"
    />
    <Dialog.Content
      class="bg-surface data-[state=open]:animate-sheet-in data-[state=closed]:animate-sheet-out pb-safe fixed inset-x-0 bottom-0 z-50 flex max-h-[85dvh] flex-col rounded-t-3xl outline-none"
    >
      <div class="flex items-center justify-between px-5 pt-4 pb-2">
        <Dialog.Title class="text-lg font-bold">Рецепт от шефа</Dialog.Title>
        <Dialog.Close class="text-ink-muted flex size-9 items-center justify-center rounded-full" aria-label="Закрыть">
          <X size={20} />
        </Dialog.Close>
      </div>
      <Dialog.Description class="sr-only">Генерация рецепта из выбранных продуктов</Dialog.Description>

      <div class="scroll-y min-h-0 flex-1 px-5 pb-4">
        {#if recipe}
          <RecipeCard {recipe} />
        {:else}
          <p class="text-ink-muted mb-3 text-sm">Выбрано продуктов: {ingredients.count}</p>
          <ul class="flex flex-wrap gap-1.5">
            {#each ingredients.selected as item (item.id)}
              <li class="bg-brand-100 text-brand-700 rounded-full px-3 py-1 text-xs font-medium">
                {item.emoji} {item.name}
              </li>
            {/each}
          </ul>
        {/if}

        {#if error}
          <p class="mt-3 text-sm text-red-600">{error}</p>
        {/if}
      </div>

      <div class="px-5 pt-2 pb-4">
        {#if !settings.isConfigured}
          <Button class="w-full" variant="secondary" onclick={() => { open = false; navigation.go('settings'); }}>
            Сначала настройте API
          </Button>
        {:else}
          <Button class="w-full" disabled={loading || ingredients.count === 0} onclick={generate}>
            <Sparkles size={20} />
            {loading ? 'Шеф думает…' : recipe ? 'Сгенерировать заново' : 'Сгенерировать рецепт'}
          </Button>
        {/if}
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
