import { computed, ref, type Ref } from "vue";
import type { FilterType, PasswordEntry } from "../types/password";
import { isRecentlyUpdated } from "../utils/passwordDates";

export function usePasswordFilters(entries: Ref<PasswordEntry[]>) {
  const searchQuery = ref("");
  const activeFilter = ref<FilterType>("all");

  const filteredEntries = computed(() => {
    const now = new Date();
    let result = entries.value;

    switch (activeFilter.value) {
      case "frequentlyUsed":
        result = result.filter((entry) => entry.isFrequentlyUsed);
        break;
      case "recentlyAdded":
        result = result.filter((entry) => isRecentlyUpdated(entry.updatedAt, now));
        break;
      case "with2FA":
        result = result.filter((entry) => entry.has2FA);
        break;
      case "securityIssues":
        result = result.filter((entry) => entry.isWeak || entry.isReused);
        break;
    }

    const query = searchQuery.value.toLowerCase().trim();

    if (!query) {
      return result;
    }

    return result.filter(
      (entry) =>
        entry.serviceName.toLowerCase().includes(query) ||
        entry.domain.toLowerCase().includes(query) ||
        entry.username.toLowerCase().includes(query),
    );
  });

  function clearSearch() {
    searchQuery.value = "";
  }

  return {
    searchQuery,
    activeFilter,
    filteredEntries,
    clearSearch,
  };
}
