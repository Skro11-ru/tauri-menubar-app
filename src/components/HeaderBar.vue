<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

const props = defineProps<{
  isPinned: boolean;
}>();

const emit = defineEmits<{
  togglePin: [];
}>();
const appWindow = getCurrentWindow();

async function startDrag(event: MouseEvent) {
  if (props.isPinned) {
    return;
  }
  if (event.button !== 0) {
    return;
  }

  try {
    await appWindow.startDragging();
  } catch (error) {
    console.error("Failed to start dragging window:", error);
  }
}

async function closeWindow() {
  try {
    await invoke("cmd_front_hide");
  } catch (error) {}
}
</script>

<template>
  <div class="header" @mousedown="startDrag" :class="{ '--can-move': !props.isPinned }">
    <h1 class="header-title">Passwords</h1>

    <div class="header-actions">
      <button
        class="header-action"
        :class="{ active: !props.isPinned }"
        :title="props.isPinned ? 'Открепить окно' : 'Закрепить окно'"
        @click="emit('togglePin')"
      >
        <svg viewBox="0 0 32 32" class="icon">
          <path
            v-if="!props.isPinned"
            fill="currentColor"
            d="M28.586 13.314L30 11.9L20 2l-1.314 1.415l1.186 1.186L8.38 14.322l-1.716-1.715L5.25 14l5.657 5.677L2 28.583L3.41 30l8.911-8.909L18 26.748l1.393-1.414l-1.716-1.716l9.724-11.49Z"
          />

          <template v-else>
            <path
              fill="currentColor"
              d="M28.59 13.31L30 11.9L20 2l-1.31 1.42l1.18 1.18l-11.49 9.72l-1.72-1.71L5.25 14l5.66 5.68L2 28.58L3.41 30l8.91-8.91L18 26.75l1.39-1.42l-1.71-1.71l9.72-11.49ZM16.26 22.2L9.8 15.74L21.29 6L26 10.71Z"
            />
          </template>
        </svg>
      </button>

      <button class="header-action" title="Settings">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="32"
          height="32"
          viewBox="0 0 32 32"
          class="icon"
        >
          <path
            fill="currentColor"
            d="M27 16.76v-1.53l1.92-1.68A2 2 0 0 0 29.3 11l-2.36-4a2 2 0 0 0-1.73-1a2 2 0 0 0-.64.1l-2.43.82a11 11 0 0 0-1.31-.75l-.51-2.52a2 2 0 0 0-2-1.61h-4.68a2 2 0 0 0-2 1.61l-.51 2.52a11.5 11.5 0 0 0-1.32.75l-2.38-.86A2 2 0 0 0 6.79 6a2 2 0 0 0-1.73 1L2.7 11a2 2 0 0 0 .41 2.51L5 15.24v1.53l-1.89 1.68A2 2 0 0 0 2.7 21l2.36 4a2 2 0 0 0 1.73 1a2 2 0 0 0 .64-.1l2.43-.82a11 11 0 0 0 1.31.75l.51 2.52a2 2 0 0 0 2 1.61h4.72a2 2 0 0 0 2-1.61l.51-2.52a11.5 11.5 0 0 0 1.32-.75l2.42.82a2 2 0 0 0 .64.1a2 2 0 0 0 1.73-1l2.28-4a2 2 0 0 0-.41-2.51ZM25.21 24l-3.43-1.16a8.9 8.9 0 0 1-2.71 1.57L18.36 28h-4.72l-.71-3.55a9.4 9.4 0 0 1-2.7-1.57L6.79 24l-2.36-4l2.72-2.4a8.9 8.9 0 0 1 0-3.13L4.43 12l2.36-4l3.43 1.16a8.9 8.9 0 0 1 2.71-1.57L13.64 4h4.72l.71 3.55a9.4 9.4 0 0 1 2.7 1.57L25.21 8l2.36 4l-2.72 2.4a8.9 8.9 0 0 1 0 3.13L27.57 20Z"
          />
          <path
            fill="currentColor"
            d="M16 22a6 6 0 1 1 6-6a5.94 5.94 0 0 1-6 6m0-10a3.91 3.91 0 0 0-4 4a3.91 3.91 0 0 0 4 4a3.91 3.91 0 0 0 4-4a3.91 3.91 0 0 0-4-4"
          />
        </svg>
      </button>

      <button class="header-action close-button" title="Закрыть окно" @click="closeWindow">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="32"
          height="32"
          viewBox="0 0 32 32"
          class="icon"
        >
          <!-- Icon from Carbon by IBM - undefined -->
          <path
            fill="currentColor"
            d="M16 2C8.2 2 2 8.2 2 16s6.2 14 14 14s14-6.2 14-14S23.8 2 16 2m0 26C9.4 28 4 22.6 4 16S9.4 4 16 4s12 5.4 12 12s-5.4 12-12 12"
          />
          <path
            fill="currentColor"
            d="M21.4 23L16 17.6L10.6 23L9 21.4l5.4-5.4L9 10.6L10.6 9l5.4 5.4L21.4 9l1.6 1.6l-5.4 5.4l5.4 5.4z"
          />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 12px 8px;
  -webkit-app-region: drag;
}

.header.--can-move {
  cursor: grab;
}

.header-title {
  margin: 0;
  color: var(--text-primary);
  font-size: 18px;
  font-weight: 600;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 4px;
  -webkit-app-region: no-drag;
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
  cursor: pointer;
  transition: background 0.15s ease;
  padding: 0;
}

.header-action:hover {
  background: var(--surface-hover);
}

.icon {
  display: block;
  width: 20px;
  height: 20px;
}

.icon path {
  fill: var(--text-tertiary);
  transition:
    fill 0.15s ease,
    stroke 0.15s ease;
}

.stroke-icon {
  fill: none !important;
  stroke: var(--text-tertiary);
  stroke-width: 1.5;
  stroke-linecap: round;
}

.header-action:hover .icon path {
  fill: var(--text-primary);
}

.header-action:hover .stroke-icon {
  stroke: var(--text-primary);
}

.header-action.active .icon path {
  fill: #007aff;
}

.header-action.active .stroke-icon {
  stroke: #007aff;
}

.close-button:hover .icon path {
  fill: #ff5f56;
}
</style>
