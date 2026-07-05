import { ref, watch, type ComputedRef } from "vue";
import { setBadgeCount } from "../services/tauriCommands";

export function useBadgeCount(count: ComputedRef<number>) {
  const badgeError = ref<string | null>(null);

  watch(
    count,
    async (value) => {
      try {
        await setBadgeCount(value);
        badgeError.value = null;
      } catch (error) {
        badgeError.value = "Не удалось обновить бейдж";
        console.error("Failed to update badge count:", error);
      }
    },
    { immediate: true },
  );

  return {
    badgeError,
  };
}
