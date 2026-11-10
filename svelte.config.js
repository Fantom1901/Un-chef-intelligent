import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

export default {
  preprocess: vitePreprocess(),
  vitePlugin: {
    // Руны принудительно только для нашего кода: lucide-svelte в node_modules собран в legacy-режиме.
    dynamicCompileOptions({ filename }) {
      if (!filename.includes('node_modules')) return { runes: true };
    }
  }
};
