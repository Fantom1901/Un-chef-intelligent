<script lang="ts">
  import type { Ingredient } from '$lib/types/recipe';

  let {
    ingredient,
    selected,
    onToggle
  }: { ingredient: Ingredient; selected: boolean; onToggle: (id: string) => void } = $props();
</script>

<button
  type="button"
  class="flex aspect-square flex-col items-center justify-center gap-1 rounded-2xl border-2 p-1 text-center transition-colors
    {selected ? 'border-brand-500 bg-brand-50' : 'border-line bg-surface-raised'}"
  aria-pressed={selected}
  onclick={() => onToggle(ingredient.id)}
>
  <span class="text-3xl leading-none">{ingredient.emoji}</span>
  <span class="line-clamp-2 text-[11px] leading-tight font-medium">{ingredient.name}</span>
</button>
