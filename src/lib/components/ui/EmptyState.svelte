<script lang="ts">
  import type { Snippet } from 'svelte';

  let { title, description, icon }: { title: string; description?: string; icon?: Snippet } = $props();
</script>

<div class="flex flex-col items-center justify-center gap-3 px-8 py-16 text-center">
  {#if icon}
    <div class="bg-brand-100 text-brand-600 flex size-16 items-center justify-center rounded-full">
      {@render icon()}
    </div>
  {/if}
  <h2 class="text-lg font-semibold">{title}</h2>
  {#if description}
    <p class="text-ink-muted text-sm">{description}</p>
  {/if}
</div>
