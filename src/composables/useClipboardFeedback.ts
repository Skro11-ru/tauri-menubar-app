import { ref } from "vue";

const FEEDBACK_TIMEOUT_MS = 1000;

export function useClipboardFeedback() {
  const copiedEntryId = ref<string | null>(null);
  const copiedFieldName = ref<string | null>(null);
  const copyError = ref<string | null>(null);

  let resetTimer: ReturnType<typeof setTimeout> | null = null;

  async function copyToClipboard(text: string, entryId: string, fieldName: string): Promise<boolean> {
    if (resetTimer) {
      clearTimeout(resetTimer);
      resetTimer = null;
    }

    try {
      await navigator.clipboard.writeText(text);
      copiedEntryId.value = entryId;
      copiedFieldName.value = fieldName;
      copyError.value = null;

      resetTimer = setTimeout(() => {
        copiedEntryId.value = null;
        copiedFieldName.value = null;
        resetTimer = null;
      }, FEEDBACK_TIMEOUT_MS);

      return true;
    } catch (error) {
      copiedEntryId.value = null;
      copiedFieldName.value = null;
      copyError.value = "Не удалось скопировать значение";
      console.error("Failed to copy value:", error);

      return false;
    }
  }

  return {
    copiedEntryId,
    copiedFieldName,
    copyError,
    copyToClipboard,
  };
}
