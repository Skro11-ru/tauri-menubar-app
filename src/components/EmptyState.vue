<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  variant: "search" | "empty";
}>();

const title = computed(() =>
  props.variant === "search" ? "No results found" : "No saved items yet",
);
const subtitle = computed(() =>
  props.variant === "search"
    ? "Try a different title or website"
    : "Your passwords will appear here once a data source is connected",
);
</script>

<template>
  <div class="empty-state">
    <div class="empty-icon">
      <svg width="40" height="40" viewBox="0 0 16 16" fill="none">
        <path
          v-if="props.variant === 'search'"
          d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"
          fill="currentColor"
          opacity="0.3"
        />
        <path
          v-else
          d="M8 1a2 2 0 0 1 2 2v4H6V3a2 2 0 0 1 2-2zm3 6V3a3 3 0 0 0-6 0v4a2 2 0 0 0-2 2v5a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2z"
          fill="currentColor"
          opacity="0.3"
        />
      </svg>
    </div>
    <div class="empty-title">{{ title }}</div>
    <div class="empty-subtitle">{{ subtitle }}</div>
  </div>
</template>

<style scoped>
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
