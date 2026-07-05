<script setup lang="ts">
import { ref } from "vue";
import { useBadgeCount } from "./composables/useBadgeCount";
import { useClipboardFeedback } from "./composables/useClipboardFeedback";
import { usePasswordEntries } from "./composables/usePasswordEntries";
import { usePasswordFilters } from "./composables/usePasswordFilters";
import { useWindowControls } from "./composables/useWindowControls";
import HeaderBar from "./components/HeaderBar.vue";
import SearchField from "./components/SearchField.vue";
import FilterRow from "./components/FilterRow.vue";
import PasswordList from "./components/PasswordList.vue";
import type { FilterType } from "./types/password";

const expandedId = ref<string | null>(null);
const { entries, weakCount } = usePasswordEntries();
const { searchQuery, activeFilter, filteredEntries, clearSearch } = usePasswordFilters(entries);
const { copiedEntryId, copiedFieldName, copyToClipboard } = useClipboardFeedback();
const { isPinned, startDragging, togglePin, closeWindow } = useWindowControls();

useBadgeCount(weakCount);

function setFilter(filter: FilterType) {
  activeFilter.value = filter;
}

function toggleExpand(id: string) {
  expandedId.value = expandedId.value === id ? null : id;
}
</script>

<template>
  <div class="app-container">
    <!-- Header -->
    <HeaderBar
      :is-pinned="isPinned"
      @toggle-pin="togglePin"
      @close="closeWindow"
      @start-drag="startDragging"
    />

    <!-- Search -->
    <div class="search-wrapper">
      <SearchField
        v-model="searchQuery"
        placeholder="Search by title or website"
        @clear="clearSearch"
      />
    </div>

    <!-- Filters -->
    <div class="filters-wrapper">
      <FilterRow :active="activeFilter" @change="setFilter" />
    </div>

    <PasswordList
      :entries="filteredEntries"
      :search-query="searchQuery"
      :expanded-id="expandedId"
      :copied-entry-id="copiedEntryId"
      :copied-field-name="copiedFieldName"
      @toggle="toggleExpand"
      @copy="copyToClipboard"
    />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--bg-color);
  border-radius: 12px;
  padding-left: 12px;
  padding-right: 12px;
  gap: 6px;
}
</style>
