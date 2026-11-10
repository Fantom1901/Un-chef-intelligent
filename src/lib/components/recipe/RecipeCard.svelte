<script lang="ts">
  import Clock from 'lucide-svelte/icons/clock';
  import Lightbulb from 'lucide-svelte/icons/lightbulb';
  import type { RecipeResponse } from '$lib/types/recipe';

  let { recipe }: { recipe: RecipeResponse } = $props();
</script>

<article class="select-allow flex flex-col gap-4">
  <div>
    <h3 class="text-xl font-bold">{recipe.title}</h3>
    <p class="text-ink-muted mt-1 flex items-center gap-1.5 text-sm">
      <Clock size={16} />
      {recipe.cookingTimeMinutes} мин
    </p>
  </div>

  <section>
    <h4 class="mb-1.5 text-sm font-semibold">Использованные продукты</h4>
    <ul class="flex flex-wrap gap-1.5">
      {#each recipe.usedIngredients as item (item)}
        <li class="bg-brand-100 text-brand-700 rounded-full px-3 py-1 text-xs font-medium">{item}</li>
      {/each}
    </ul>
  </section>

  {#if recipe.missingIngredients.length > 0}
    <section>
      <h4 class="mb-1.5 text-sm font-semibold">Докупить</h4>
      <ul class="flex flex-wrap gap-1.5">
        {#each recipe.missingIngredients as item (item)}
          <li class="bg-line text-ink rounded-full px-3 py-1 text-xs font-medium">{item}</li>
        {/each}
      </ul>
    </section>
  {/if}

  <section>
    <h4 class="mb-1.5 text-sm font-semibold">Приготовление</h4>
    <ol class="flex flex-col gap-3">
      {#each recipe.steps as step, i (i)}
        <li class="flex gap-3 text-sm">
          <span class="bg-brand-500 flex size-6 shrink-0 items-center justify-center rounded-full text-xs font-bold text-white">
            {i + 1}
          </span>
          <span>{step}</span>
        </li>
      {/each}
    </ol>
  </section>

  <aside class="bg-brand-50 flex gap-2 rounded-2xl p-3 text-sm">
    <Lightbulb size={18} class="text-brand-600 mt-0.5 shrink-0" />
    <p><span class="font-semibold">Совет шефа:</span> {recipe.chefTip}</p>
  </aside>
</article>
