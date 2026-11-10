import type { Ingredient, IngredientCategory } from '$lib/types/recipe';

const builtin = (id: string, name: string, emoji: string, category: IngredientCategory): Ingredient => ({
  id,
  name,
  emoji,
  category,
  isCustom: false
});

export const DEFAULT_INGREDIENTS: Ingredient[] = [
  builtin('potato', 'Картофель', '🥔', 'vegetables'),
  builtin('carrot', 'Морковь', '🥕', 'vegetables'),
  builtin('onion', 'Лук', '🧅', 'vegetables'),
  builtin('garlic', 'Чеснок', '🧄', 'vegetables'),
  builtin('tomato', 'Помидоры', '🍅', 'vegetables'),
  builtin('cucumber', 'Огурцы', '🥒', 'vegetables'),
  builtin('pepper', 'Перец', '🫑', 'vegetables'),
  builtin('cabbage', 'Капуста', '🥬', 'vegetables'),
  builtin('mushroom', 'Грибы', '🍄', 'vegetables'),
  builtin('apple', 'Яблоки', '🍎', 'fruits'),
  builtin('lemon', 'Лимон', '🍋', 'fruits'),
  builtin('chicken', 'Курица', '🍗', 'meat'),
  builtin('beef', 'Говядина', '🥩', 'meat'),
  builtin('sausage', 'Колбаса', '🌭', 'meat'),
  builtin('fish', 'Рыба', '🐟', 'fish'),
  builtin('egg', 'Яйца', '🥚', 'dairy'),
  builtin('milk', 'Молоко', '🥛', 'dairy'),
  builtin('cheese', 'Сыр', '🧀', 'dairy'),
  builtin('butter', 'Масло сливочное', '🧈', 'dairy'),
  builtin('rice', 'Рис', '🍚', 'grains'),
  builtin('pasta', 'Макароны', '🍝', 'grains'),
  builtin('bread', 'Хлеб', '🍞', 'grains'),
  builtin('flour', 'Мука', '🌾', 'grains'),
  builtin('beans', 'Фасоль', '🫘', 'pantry')
];

class IngredientsState {
  catalog = $state<Ingredient[]>([...DEFAULT_INGREDIENTS]);
  selectedIds = $state<string[]>([]);

  selected = $derived(this.catalog.filter((i) => this.selectedIds.includes(i.id)));
  selectedNames = $derived(this.selected.map((i) => i.name));
  count = $derived(this.selectedIds.length);

  isSelected(id: string): boolean {
    return this.selectedIds.includes(id);
  }

  toggle(id: string) {
    this.selectedIds = this.isSelected(id)
      ? this.selectedIds.filter((x) => x !== id)
      : [...this.selectedIds, id];
  }

  addCustom(name: string, emoji = '🍽️'): Ingredient | null {
    const trimmed = name.trim();
    if (!trimmed) return null;

    const existing = this.catalog.find((i) => i.name.toLowerCase() === trimmed.toLowerCase());
    if (existing) {
      if (!this.isSelected(existing.id)) this.toggle(existing.id);
      return existing;
    }

    const item: Ingredient = {
      id: `custom-${crypto.randomUUID()}`,
      name: trimmed,
      emoji,
      category: 'other',
      isCustom: true
    };
    this.catalog = [...this.catalog, item];
    this.selectedIds = [...this.selectedIds, item.id];
    return item;
  }

  clearSelection() {
    this.selectedIds = [];
  }
}

export const ingredients = new IngredientsState();
