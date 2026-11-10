<script lang="ts">
  import { onMount } from 'svelte';
  import SafeAreaContainer from '$lib/components/layout/SafeAreaContainer.svelte';
  import BottomNav from '$lib/components/layout/BottomNav.svelte';
  import ProductsScreen from '$lib/components/products/ProductsScreen.svelte';
  import HistoryScreen from '$lib/components/history/HistoryScreen.svelte';
  import SettingsScreen from '$lib/components/settings/SettingsScreen.svelte';
  import RecipeSheet from '$lib/components/recipe/RecipeSheet.svelte';
  import { navigation } from '$lib/state/navigation.svelte';
  import { settings } from '$lib/state/settings.svelte';

  onMount(() => {
    void settings.load();
    return navigation.watchKeyboard();
  });
</script>

<SafeAreaContainer>
  <main class="scroll-y min-h-0 flex-1">
    {#if navigation.tab === 'products'}
      <ProductsScreen />
    {:else if navigation.tab === 'history'}
      <HistoryScreen />
    {:else}
      <SettingsScreen />
    {/if}
  </main>

  {#if !navigation.keyboardOpen}
    <BottomNav />
  {/if}
</SafeAreaContainer>

<RecipeSheet bind:open={navigation.recipeSheetOpen} />
