<script lang="ts">
  import ShoppingBasket from 'lucide-svelte/icons/shopping-basket';
  import History from 'lucide-svelte/icons/history';
  import Settings from 'lucide-svelte/icons/settings';
  import type { Component } from 'svelte';
  import { navigation } from '$lib/state/navigation.svelte';
  import type { TabId } from '$lib/types/recipe';

  // lucide-svelte экспортирует иконки как компоненты, поэтому типизируем свободно.
  const tabs: { id: TabId; label: string; icon: Component<any> }[] = [
    { id: 'products', label: 'Продукты', icon: ShoppingBasket as unknown as Component<any> },
    { id: 'history', label: 'История', icon: History as unknown as Component<any> },
    { id: 'settings', label: 'Настройки', icon: Settings as unknown as Component<any> }
  ];
</script>

<nav class="border-line bg-surface-raised pb-safe shrink-0 border-t" aria-label="Основная навигация">
  <ul class="flex h-nav items-stretch">
    {#each tabs as tab (tab.id)}
      {@const active = navigation.tab === tab.id}
      <li class="flex-1">
        <button
          type="button"
          class="flex h-full w-full flex-col items-center justify-center gap-0.5 text-xs font-medium transition-colors
            {active ? 'text-brand-600' : 'text-ink-muted'}"
          aria-current={active ? 'page' : undefined}
          onclick={() => navigation.go(tab.id)}
        >
          <tab.icon size={22} strokeWidth={active ? 2.5 : 2} />
          <span>{tab.label}</span>
        </button>
      </li>
    {/each}
  </ul>
</nav>
