import { onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function useWindowFocus(onFocus: () => Promise<void> | void) {
  const appWindow = getCurrentWindow();
  let unlistenFocus: null | (() => void) = null;

  onMounted(async () => {
    try {
      unlistenFocus = await appWindow.onFocusChanged(async ({ payload: focused }) => {
        if (focused) {
          await onFocus();
        }
      });
    } catch (error) {
      console.error("Failed to subscribe to window focus changes:", error);
    }
  });

  onUnmounted(() => {
    unlistenFocus?.();
    unlistenFocus = null;
  });
}
