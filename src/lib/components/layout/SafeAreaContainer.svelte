<script lang="ts">
  import type { Snippet } from 'svelte';

  let { children }: { children: Snippet } = $props();
</script>

<!-- Верхний inset учитывается здесь; нижний — в BottomNav (прячется вместе с клавиатурой). -->
<div class="h-app pt-safe px-safe bg-surface flex flex-col overflow-hidden">
  {@render children()}
</div>
