import { ref, computed, type Ref } from "vue";
import { mockPasswords } from "../data/mockPasswords";
import type { PasswordEntry, FilterType } from "../types/password";

export function usePasswords() {
  const entries = ref<PasswordEntry[]>(mockPasswords) as Ref<PasswordEntry[]>;
  const searchQuery = ref("");
  const activeFilter = ref<FilterType>("all");
  const expandedId = ref<string | null>(null);
  const copiedEntryId = ref<string | null>(null);
  const copiedFieldName = ref<string | null>(null);

  const filteredEntries = computed(() => {
    let result = entries.value;

    // Apply filter
    switch (activeFilter.value) {
      case "frequentlyUsed":
        result = result.filter((e) => e.isFrequentlyUsed);
        break;
      case "recentlyAdded":
        result = result.filter(
          (e) =>
            e.lastUpdated.includes("h") ||
            e.lastUpdated.includes("1d") ||
            e.lastUpdated.includes("2d"),
        );
        break;
      case "with2FA":
        result = result.filter((e) => e.has2FA);
        break;
      case "securityIssues":
        result = result.filter((e) => e.isWeak || e.isReused);
        break;
    }

    // Apply search
    const query = searchQuery.value.toLowerCase().trim();
    if (query) {
      result = result.filter(
        (e) =>
          e.serviceName.toLowerCase().includes(query) ||
          e.domain.toLowerCase().includes(query) ||
          e.username.toLowerCase().includes(query),
      );
    }

    return result;
  });

  function toggleExpand(id: string) {
    expandedId.value = expandedId.value === id ? null : id;
  }

  async function copyToClipboard(text: string, entryId: string, field: string) {
    try {
      await navigator.clipboard.writeText(text);
      copiedEntryId.value = entryId;
      copiedFieldName.value = field;
      setTimeout(() => {
        copiedEntryId.value = null;
        copiedFieldName.value = null;
      }, 1000);
    } catch {
      // Copy failed silently - user will retry
    }
  }

  function clearSearch() {
    searchQuery.value = "";
  }

  return {
    entries: filteredEntries,
    searchQuery,
    activeFilter,
    expandedId,
    copiedEntryId,
    copiedFieldName,
    toggleExpand,
    copyToClipboard,
    clearSearch,
  };
}
