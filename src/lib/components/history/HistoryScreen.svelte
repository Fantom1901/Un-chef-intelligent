<script lang="ts">
  import History from 'lucide-svelte/icons/history';
  import ScreenHeader from '$lib/components/ui/ScreenHeader.svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
</script>

<ScreenHeader title="История" subtitle="Ваши сохранённые рецепты" />
<EmptyState title="Пока пусто" description="Сгенерированные рецепты появятся здесь.">
  {#snippet icon()}<History size={28} />{/snippet}
</EmptyState>
