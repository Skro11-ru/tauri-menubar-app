<script setup lang="ts">
import { usePasswords } from "./composables/usePasswords";
import HeaderBar from "./components/HeaderBar.vue";
import SearchField from "./components/SearchField.vue";
import FilterRow from "./components/FilterRow.vue";
import PasswordCard from "./components/PasswordCard.vue";
import type { FilterType } from "./types/password";

const {
  entries,
  searchQuery,
  activeFilter,
  expandedId,
  copiedEntryId,
  copiedFieldName,
  toggleExpand,
  copyToClipboard,
  clearSearch,
} = usePasswords();
</script>

<template>
  <div class="app-container">
    <!-- Header -->
    <HeaderBar />

    <!-- Search -->
    <div class="search-wrapper">
      <SearchField
        v-model="searchQuery"
        placeholder="Search by title or website"
        :has-results="entries.length"
        @clear="clearSearch"
      />
    </div>

    <!-- Filters -->
    <div class="filters-wrapper">
      <FilterRow :active="activeFilter" @change="(filter: FilterType) => (activeFilter = filter)" />
    </div>

    <!-- Password list -->
    <div class="password-list">
      <!-- Empty search result -->
      <div v-if="entries.length === 0 && searchQuery" class="empty-state">
        <div class="empty-icon">
          <svg width="40" height="40" viewBox="0 0 16 16" fill="none">
            <path
              d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"
              fill="currentColor"
              opacity="0.3"
            />
          </svg>
        </div>
        <div class="empty-title">No results found</div>
        <div class="empty-subtitle">Try a different title or website</div>
      </div>

      <!-- Full empty state -->
      <div v-else-if="entries.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg width="40" height="40" viewBox="0 0 16 16" fill="none">
            <path
              d="M8 1a2 2 0 0 1 2 2v4H6V3a2 2 0 0 1 2-2zm3 6V3a3 3 0 0 0-6 0v4a2 2 0 0 0-2 2v5a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2z"
              fill="currentColor"
              opacity="0.3"
            />
          </svg>
        </div>
        <div class="empty-title">No saved items yet</div>
        <div class="empty-subtitle">
          Your passwords will appear here once a data source is connected
        </div>
      </div>

      <!-- Password cards -->
      <PasswordCard
        v-for="entry in entries"
        :key="entry.id"
        :entry="entry"
        :is-expanded="expandedId === entry.id"
        :copied-entry-id="copiedEntryId"
        :copied-field-name="copiedFieldName"
        @toggle="toggleExpand(entry.id)"
        @copy="(text: string, field: string) => copyToClipboard(text, entry.id, field)"
      />
    </div>
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

.password-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  padding-bottom: 12px;
  gap: 8px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  text-align: center;
}

.empty-icon {
  margin-bottom: 12px;
  color: var(--text-tertiary);
}

.empty-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.empty-subtitle {
  font-size: 12px;
  color: var(--text-tertiary);
}
</style>
