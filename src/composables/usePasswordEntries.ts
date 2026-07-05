import { computed, ref } from "vue";
import { mockPasswords } from "../data/mockPasswords";
import type { PasswordEntry } from "../types/password";

export function usePasswordEntries() {
  const entries = ref<PasswordEntry[]>(mockPasswords);
  const weakCount = computed(() => entries.value.filter((entry) => entry.isWeak).length);

  return {
    entries,
    weakCount,
  };
}
