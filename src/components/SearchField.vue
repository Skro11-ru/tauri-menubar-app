<script setup lang="ts">
const props = defineProps<{
  modelValue: string;
  placeholder?: string;
  hasResults?: number;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: string];
  clear: [];
}>();

function onInput(event: Event) {
  const target = event.target as HTMLInputElement;
  emit("update:modelValue", target.value);
}
</script>

<template>
  <div class="search-field">
    <div class="search-icon">
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
        <path
          d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"
          fill="currentColor"
          opacity="0.5"
        />
      </svg>
    </div>
    <input
      type="text"
      class="search-input"
      :value="modelValue"
      @input="onInput"
      :placeholder="placeholder"
    />
    <button v-if="modelValue" class="clear-button" @click="$emit('clear')">
      <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
        <path
          d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"
          fill="currentColor"
        />
      </svg>
    </button>
  </div>
</template>

<style scoped>
.search-field {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 10px;
  display: flex;
  align-items: center;
  pointer-events: none;
  color: var(--text-secondary);
}

.search-input {
  width: 100%;
  padding: 8px 32px 8px 32px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--surface-secondary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  transition: all 0.15s ease;
}

.search-input:focus {
  border-color: var(--accent-color);
  background: var(--surface-base);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.search-input::placeholder {
  color: var(--text-tertiary);
}

.clear-button {
  padding: 0;
  position: absolute;
  right: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border: none;
  border-radius: 50%;
  background: var(--text-tertiary);
  color: var(--surface-base);
  cursor: pointer;
  opacity: 0.6;
  transition: opacity 0.15s ease;
}

.clear-button:hover {
  opacity: 1;
}
</style>
