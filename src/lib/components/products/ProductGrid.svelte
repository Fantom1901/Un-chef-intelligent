<script lang="ts">
  import IngredientChip from './IngredientChip.svelte';
  import { ingredients } from '$lib/state/ingredients.svelte';
</script>

<div class="grid grid-cols-3 gap-2.5 px-5 pb-4">
  {#each ingredients.catalog as item (item.id)}
    <IngredientChip
      ingredient={item}
      selected={ingredients.isSelected(item.id)}
      onToggle={(id) => ingredients.toggle(id)}
    />
  {/each}
</div>
