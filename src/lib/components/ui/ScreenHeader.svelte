<script lang="ts">
  let { title, subtitle }: { title: string; subtitle?: string } = $props();
</script>

<header class="px-5 pt-5 pb-3">
  <h1 class="text-2xl font-bold tracking-tight">{title}</h1>
  {#if subtitle}
    <p class="text-ink-muted mt-1 text-sm">{subtitle}</p>
  {/if}
</header>
