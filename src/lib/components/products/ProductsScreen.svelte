<script lang="ts">
  import ChefHat from 'lucide-svelte/icons/chef-hat';
  import ScreenHeader from '$lib/components/ui/ScreenHeader.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import ProductGrid from './ProductGrid.svelte';
  import AddCustomIngredient from './AddCustomIngredient.svelte';
  import { ingredients } from '$lib/state/ingredients.svelte';
  import { navigation } from '$lib/state/navigation.svelte';
</script>

<div class="flex min-h-full flex-col">
  <ScreenHeader title="Продукты" subtitle="Отметьте, что есть дома" />
  <AddCustomIngredient />
  <div class="flex-1">
    <ProductGrid />
  </div>

  <div class="bg-surface/95 sticky bottom-0 px-5 py-3 backdrop-blur">
    <Button class="w-full" disabled={ingredients.count === 0} onclick={() => navigation.openRecipeSheet()}>
      <ChefHat size={20} />
      {ingredients.count === 0 ? 'Выберите продукты' : `Приготовить (${ingredients.count})`}
    </Button>
  </div>
</div>
