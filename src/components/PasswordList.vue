<script setup lang="ts">
import PasswordCard from "./PasswordCard.vue";
import EmptyState from "./EmptyState.vue";
import type { PasswordEntry } from "../types/password";

defineProps<{
  entries: PasswordEntry[];
  searchQuery: string;
  expandedId: string | null;
  copiedEntryId: string | null;
  copiedFieldName: string | null;
}>();

const emit = defineEmits<{
  toggle: [id: string];
  copy: [text: string, entryId: string, fieldName: string];
}>();
</script>

<template>
  <div class="password-list">
    <EmptyState v-if="entries.length === 0" :variant="searchQuery ? 'search' : 'empty'" />

    <PasswordCard
      v-for="entry in entries"
      :key="entry.id"
      :entry="entry"
      :is-expanded="expandedId === entry.id"
      :copied-entry-id="copiedEntryId"
      :copied-field-name="copiedFieldName"
      @toggle="emit('toggle', entry.id)"
      @copy="(text, fieldName) => emit('copy', text, entry.id, fieldName)"
    />
  </div>
</template>

<style scoped>
.password-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  padding-bottom: 12px;
  gap: 8px;
}
</style>
