<template>
  <div class="modal-backdrop" @click.self="$emit('close')">
    <div class="modal">
      <h2>{{ dateLabel }}</h2>

      <div class="status-toggle">
        <button
          class="toggle-btn pass"
          :class="{ active: status === 1 }"
          @click="status = 1"
        >
          ✅ 완료
        </button>
        <button
          class="toggle-btn fail"
          :class="{ active: status === 0 }"
          @click="status = 0"
        >
          ❌ 실패
        </button>
      </div>

      <label class="field">
        <span>메모 (선택)</span>
        <textarea v-model="note" rows="3" placeholder="오늘 어땠나요?" />
      </label>

      <div class="modal-actions">
        <button class="btn" @click="$emit('close')">취소</button>
        <button class="btn btn-primary" @click="save">저장</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { BlptCell } from '../api/types'

const props = defineProps<{ cell: BlptCell }>()
const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', payload: { status: number; note: string | null }): void
}>()

const status = ref(props.cell.status === 'pass' ? 1 : 0)
const note = ref(props.cell.note ?? '')

const dateLabel = new Date(props.cell.date * 1000).toLocaleDateString('ko-KR', {
  year: 'numeric',
  month: 'long',
  day: 'numeric'
})

function save() {
  emit('save', { status: status.value, note: note.value.trim() || null })
}
</script>

<style scoped>
.modal-backdrop {
  position: fixed;
  inset: 0;
  background-color: #00000066;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal {
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: 24px;
  width: 320px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
}

.modal h2 {
  color: var(--color-text-bright);
}

.status-toggle {
  display: flex;
  gap: 8px;
}

.toggle-btn {
  flex: 1;
  padding: 10px;
  border-radius: var(--radius-sm);
  border: 2px solid var(--color-border);
  background-color: var(--color-bg);
  color: var(--color-text);
  cursor: pointer;
  font-size: 0.95rem;
}

.toggle-btn.pass.active {
  border-color: var(--color-pass);
  background-color: color-mix(in srgb, var(--color-pass) 25%, var(--color-bg));
  color: var(--color-text-bright);
}

.toggle-btn.fail.active {
  border-color: var(--color-fail);
  background-color: color-mix(in srgb, var(--color-fail) 25%, var(--color-bg));
  color: var(--color-text-bright);
}

.field {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 0.9rem;
  color: var(--color-text-muted);
}

.field textarea {
  padding: 6px 8px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background-color: var(--color-bg);
  font-size: 1rem;
  font-family: inherit;
  color: var(--color-text);
  resize: vertical;
}

.field textarea:focus {
  outline: none;
  border-color: var(--color-accent);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 8px;
}

.btn {
  padding: 6px 14px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border);
  background-color: var(--color-surface-hover);
  color: var(--color-text);
  cursor: pointer;
}

.btn:hover {
  border-color: var(--color-accent);
}

.btn-primary {
  background-color: var(--color-pass);
  color: white;
  border-color: var(--color-pass);
}

.btn-primary:hover {
  background-color: var(--color-pass-strong);
  border-color: var(--color-pass-strong);
}
</style>
