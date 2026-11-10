<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';

  interface Props extends HTMLButtonAttributes {
    variant?: 'primary' | 'secondary' | 'ghost';
    children: Snippet;
  }

  let { variant = 'primary', class: className = '', children, ...rest }: Props = $props();

  const variants = {
    primary: 'bg-brand-500 text-white active:bg-brand-600 disabled:bg-brand-200',
    secondary: 'bg-brand-100 text-brand-700 active:bg-brand-200 disabled:opacity-50',
    ghost: 'bg-transparent text-ink-muted active:bg-line disabled:opacity-50'
  } as const;
</script>

<button
  type="button"
  class="inline-flex h-12 items-center justify-center gap-2 rounded-2xl px-5 text-base font-semibold transition-colors disabled:cursor-not-allowed {variants[variant]} {className}"
  {...rest}
>
  {@render children()}
</button>
