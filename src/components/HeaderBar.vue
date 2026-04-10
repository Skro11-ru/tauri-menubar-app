<script setup lang="ts">
// Header is minimal - just title and optional settings action
</script>

<template>
  <div class="header">
    <h1 class="header-title">Passwords</h1>
    <button class="header-action" title="Settings">
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M7.404 1.594a.75.75 0 0 1 1.192 0l.468.54a.75.75 0 0 0 .658.262l.717-.09a.75.75 0 0 1 .79.568l.156.707a.75.75 0 0 0 .49.552l.67.232a.75.75 0 0 1 .49.726v.726a.75.75 0 0 1-.49.726l-.67.232a.75.75 0 0 0-.49.552l-.156.707a.75.75 0 0 1-.79.568l-.717-.09a.75.75 0 0 0-.658.262l-.468.54a.75.75 0 0 1-1.192 0l-.468-.54a.75.75 0 0 0-.658-.262l-.717.09a.75.75 0 0 1-.79-.568l-.156-.707a.75.75 0 0 0-.49-.552l-.67-.232a.75.75 0 0 1-.49-.726v-.726a.75.75 0 0 1 .49-.726l.67-.232a.75.75 0 0 0 .49-.552l.156-.707a.75.75 0 0 1 .79-.568l.717.09a.75.75 0 0 0 .658-.262l.468-.54z" fill="currentColor"/>
        <path d="M8 11a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" fill="currentColor"/>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 12px 8px;
}

.header-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.header-action {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  transition: all 0.15s ease;
}

.header-action:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}
</style>
