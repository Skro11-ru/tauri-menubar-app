<script setup lang="ts">
import type { FilterType } from "../types/password";

const filters: { key: FilterType; label: string }[] = [
  { key: "all", label: "All" },
  { key: "frequentlyUsed", label: "Frequent" },
  { key: "recentlyAdded", label: "Recent" },
  { key: "with2FA", label: "2FA" },
  { key: "securityIssues", label: "Issues" },
];

const props = defineProps<{
  active: FilterType;
}>();

const emit = defineEmits<{
  change: [filter: FilterType];
}>();
</script>

<template>
  <div class="filter-row">
    <button
      v-for="filter in filters"
      :key="filter.key"
      class="filter-chip"
      :class="{ active: active === filter.key }"
      @click="$emit('change', filter.key)"
    >
      {{ filter.label }}
    </button>
  </div>
</template>

<style scoped>
.filter-row {
  display: flex;
  gap: 6px;
  overflow-x: auto;
  padding: 2px 0;
}

.filter-chip {
  padding: 5px 12px;
  border: 1px solid var(--border-color);
  background: var(--surface-secondary);
  color: var(--text-secondary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  white-space: nowrap;
  transition: all 0.15s ease;
}

.filter-chip:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}

.filter-chip.active {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}
</style>
