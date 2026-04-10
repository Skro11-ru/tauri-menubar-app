<script setup lang="ts">
import { computed } from "vue";
import type { PasswordEntry } from "../types/password";

const props = defineProps<{
  entry: PasswordEntry;
  isExpanded: boolean;
  copiedEntryId: string | null;
  copiedFieldName: string | null;
}>();

const emit = defineEmits<{
  toggle: [];
  copy: [text: string, field: string];
}>();

const monogram = computed(() => {
  return props.entry.serviceName.substring(0, 2).toUpperCase();
});

const monogramColor = computed(() => {
  let hash = 0;
  for (let i = 0; i < props.entry.serviceName.length; i++) {
    hash = props.entry.serviceName.charCodeAt(i) + ((hash << 5) - hash);
  }
  const h = hash % 360;
  return `hsl(${h}, 25%, 85%)`;
});

const maskedPassword = computed(() => {
  return "•".repeat(Math.min(props.entry.password.length, 16));
});

const badges = computed(() => {
  const items: { label: string; variant: string }[] = [];
  if (props.entry.has2FA) {
    items.push({ label: "2FA", variant: "neutral" });
  }
  if (props.entry.hasPasskey) {
    items.push({ label: "Passkey", variant: "neutral" });
  }
  if (props.entry.isWeak) {
    items.push({ label: "Weak", variant: "warning" });
  }
  if (props.entry.isReused) {
    items.push({ label: "Reused", variant: "warning" });
  }
  return items;
});

const isCopied = computed(() => {
  return props.copiedEntryId === props.entry.id;
});

function isFieldCopied(field: string): boolean {
  return props.copiedEntryId === props.entry.id && props.copiedFieldName === field;
}
</script>

<template>
  <div class="password-card" :class="{ expanded: isExpanded }" @click="$emit('toggle')">
    <!-- Main row -->
    <div class="card-main">
      <!-- Identity tile -->
      <div class="identity-tile" :style="{ backgroundColor: monogramColor }">
        {{ monogram }}
      </div>

      <!-- Info -->
      <div class="card-info">
        <div class="card-title">{{ entry.serviceName }}</div>
        <div class="card-domain">{{ entry.domain }}</div>
        <div class="card-username">{{ entry.username }}</div>
      </div>

      <!-- Badges -->
      <div class="card-badges">
        <span v-for="badge in badges" :key="badge.label" class="badge" :class="badge.variant">
          {{ badge.label }}
        </span>
        <span class="last-updated">{{ entry.lastUpdated }}</span>
      </div>
    </div>

    <!-- Expanded details -->
    <div v-if="isExpanded" class="expanded-content" @click.stop>
      <div class="detail-row">
        <label>Username</label>
        <button
          class="detail-value"
          :class="{ 'copy-success': isFieldCopied('username') }"
          @click="$emit('copy', entry.username, 'username')"
        >
          <span>{{ entry.username }}</span>
          <span v-if="isFieldCopied('username')" class="copy-btn"> ✓ </span>
        </button>
      </div>

      <div class="detail-row">
        <label>Password</label>

        <button
          class="detail-value password-field"
          :class="{ 'copy-success': isFieldCopied('password') }"
          @click="$emit('copy', entry.password, 'password')"
        >
          <span class="masked" :data-password="entry.password">{{ maskedPassword }}</span>
          <span v-if="isFieldCopied('password')" class="copy-btn"> ✓ </span>
        </button>
      </div>

      <div v-if="entry.notes" class="detail-notes">
        <label>Notes</label>
        <div class="notes-value">{{ entry.notes }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.password-card {
  background: var(--surface-base);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  padding: 10px 12px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.password-card:hover {
  background: var(--surface-hover);
  border-color: var(--border-hover);
}

.password-card.expanded {
  background: var(--surface-active);
  border-color: var(--accent-color);
}

.password-card .copy-success {
  animation: copyFlash 1.5s ease;
}

@keyframes copyFlash {
  0%,
  100% {
    background: var(--surface-active);
  }
  50% {
    background: var(--success-soft);
  }
}

.card-main {
  display: flex;
  align-items: flex-start;
  gap: 10px;
}

.identity-tile {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  font-weight: 600;
  /*color: var(--text-primary);*/
  flex-shrink: 0;
}

.card-info {
  flex: 1;
  min-width: 0;
  display: flex;
  gap: 3px;
  flex-direction: column;
}

.card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.card-domain {
  font-size: 12px;
  color: var(--text-secondary);
}

.card-username {
  font-size: 12px;
  color: var(--text-tertiary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-badges {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 4px;
  flex-shrink: 0;
}

.badge {
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 500;
}

.badge.neutral {
  background: var(--badge-neutral);
  color: var(--text-secondary);
}

.badge.warning {
  background: var(--badge-warning);
  color: var(--text-warning);
}

.last-updated {
  font-size: 10px;
  color: var(--text-tertiary);
}

.expanded-content {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--border-color);
}

.detail-row {
  margin-bottom: 12px;
}

.detail-row label {
  font-size: 11px;
  font-weight: 500;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 4px;
  display: block;
}

.detail-value {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  gap: 8px;
  padding: 4px 8px;
  background: var(--surface-secondary);
  border-radius: 6px;
  font-size: 12px;
  color: var(--text-primary);
}

.detail-value {
  padding: 8px 6px;
  height: 35px;
}

.detail-value:hover {
  background: rgba(255, 255, 255, 0.07);
}

.masked {
  font-family: "SF Mono", "Monaco", "Consolas", monospace;
  letter-spacing: 1px;
}

.password-field {
  position: relative;
}

.password-field:hover .masked {
  color: transparent;
}

.password-field:hover .masked::after {
  content: attr(data-password);
  position: absolute;
  left: 8px;
  top: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  color: var(--text-primary);
  font-family: "SF Mono", "Monaco", "Consolas", monospace;
  letter-spacing: 1px;
  max-width: calc(100% - 50px);
  overflow: hidden;
}

.copy-btn {
  padding-right: 8px;
}

.detail-notes {
  padding: 8px;
  background: var(--surface-secondary);
  border-radius: 6px;
}

.detail-notes label {
  font-size: 11px;
  font-weight: 500;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 4px;
  display: block;
}

.notes-value {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: "SF Mono", "Monaco", "Consolas", monospace;
}
</style>
