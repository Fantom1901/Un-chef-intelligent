import type { TabId } from '$lib/types/recipe';

class NavigationState {
  tab = $state<TabId>('products');
  /** Открыт ли bottom sheet генерации рецепта. */
  recipeSheetOpen = $state(false);
  /** Экранная клавиатура открыта — BottomNav прячется. */
  keyboardOpen = $state(false);

  go(tab: TabId) {
    this.tab = tab;
  }

  openRecipeSheet() {
    this.recipeSheetOpen = true;
  }

  /**
   * Определяет экранную клавиатуру по сжатию visualViewport относительно
   * максимальной высоты, виденной с момента старта (работает и при adjustResize).
   * Возвращает функцию отписки.
   */
  watchKeyboard(): () => void {
    const vv = window.visualViewport;
    if (!vv) return () => {};

    let baseline = vv.height;
    const update = () => {
      baseline = Math.max(baseline, vv.height);
      this.keyboardOpen = vv.height < baseline * 0.75;
    };
    const resetBaseline = () => {
      baseline = vv.height;
      this.keyboardOpen = false;
    };

    vv.addEventListener('resize', update);
    window.addEventListener('orientationchange', resetBaseline);
    return () => {
      vv.removeEventListener('resize', update);
      window.removeEventListener('orientationchange', resetBaseline);
    };
  }
}

export const navigation = new NavigationState();
