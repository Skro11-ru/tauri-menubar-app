import { ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { hideWindow, pinWindow, unpinWindow } from "../services/tauriCommands";

export function useWindowControls() {
  const isPinned = ref(false);
  const windowError = ref<string | null>(null);
  const appWindow = getCurrentWindow();

  async function startDragging(event: MouseEvent): Promise<void> {
    if (isPinned.value || event.button !== 0) {
      return;
    }

    if (event.target instanceof HTMLElement && event.target.closest("button")) {
      return;
    }

    try {
      await appWindow.startDragging();
      windowError.value = null;
    } catch (error) {
      windowError.value = "Не удалось переместить окно";
      console.error("Failed to start dragging window:", error);
    }
  }

  async function togglePin(): Promise<void> {
    try {
      if (isPinned.value) {
        await unpinWindow();
        isPinned.value = false;
      } else {
        await pinWindow();
        isPinned.value = true;
      }

      windowError.value = null;
    } catch (error) {
      windowError.value = "Не удалось изменить режим окна";
      console.error("Failed to toggle window pin state:", error);
    }
  }

  async function closeWindow(): Promise<void> {
    try {
      await hideWindow();
      windowError.value = null;
    } catch (error) {
      windowError.value = "Не удалось скрыть окно";
      console.error("Failed to hide window:", error);
    }
  }

  return {
    isPinned,
    windowError,
    startDragging,
    togglePin,
    closeWindow,
  };
}
