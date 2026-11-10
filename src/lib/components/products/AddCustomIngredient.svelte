<script lang="ts">
  import Plus from 'lucide-svelte/icons/plus';
  import { ingredients } from '$lib/state/ingredients.svelte';

  let name = $state('');

  function submit(event: SubmitEvent) {
    event.preventDefault();
    if (ingredients.addCustom(name)) name = '';
  }
</script>

<form class="flex gap-2 px-5 pb-3" onsubmit={submit}>
  <input
    type="text"
    bind:value={name}
    placeholder="Свой продукт…"
    enterkeyhint="done"
    class="border-line bg-surface-raised focus:border-brand-400 h-11 min-w-0 flex-1 rounded-xl border px-4 text-base outline-none"
  />
  <button
    type="submit"
    class="bg-brand-100 text-brand-700 flex size-11 shrink-0 items-center justify-center rounded-xl disabled:opacity-50"
    disabled={name.trim() === ''}
    aria-label="Добавить продукт"
  >
    <Plus size={22} />
  </button>
</form>
