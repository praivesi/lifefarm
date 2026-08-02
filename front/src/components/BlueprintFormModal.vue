<template>
  <div class="modal-backdrop" @click.self="$emit('close')">
    <div class="modal">
      <h2>{{ blueprint ? 'Blueprint 수정' : '새 Blueprint' }}</h2>

      <label class="field">
        <span>목표</span>
        <input v-model="form.goal" type="text" placeholder="예: 운동하기" />
      </label>

      <label class="field">
        <span>설명</span>
        <textarea v-model="form.desc" rows="3" placeholder="예: 매일 30분 걷기" />
      </label>

      <label class="field">
        <span>시작일</span>
        <input v-model="startInput" type="date" />
      </label>

      <label class="field">
        <span>종료일</span>
        <input v-model="endInput" type="date" />
      </label>

      <div class="modal-actions">
        <button class="btn" @click="$emit('close')">취소</button>
        <button class="btn btn-primary" @click="save" :disabled="!canSave">저장</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Blueprint } from '../api/types'
import type { BlueprintPayload } from '../api/blueprints'

const props = defineProps<{ blueprint: Blueprint | null }>()
const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', payload: BlueprintPayload): void
}>()

function toDateInput(timestamp: number): string {
  return new Date(timestamp * 1000).toISOString().slice(0, 10)
}

function toTimestamp(dateInput: string): number {
  return Math.floor(new Date(`${dateInput}T00:00:00+09:00`).getTime() / 1000)
}

const today = new Date().toISOString().slice(0, 10)

const form = ref({
  goal: props.blueprint?.goal ?? '',
  desc: props.blueprint?.desc ?? ''
})

const startInput = ref(props.blueprint ? toDateInput(props.blueprint.start_dt) : today)
const endInput = ref(props.blueprint ? toDateInput(props.blueprint.end_dt) : today)

const canSave = computed(() => form.value.goal.trim().length > 0 && startInput.value <= endInput.value)

function save() {
  if (!canSave.value) return

  emit('save', {
    goal: form.value.goal,
    desc: form.value.desc,
    start_dt: toTimestamp(startInput.value),
    end_dt: toTimestamp(endInput.value)
  })
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
  width: 360px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
}

.modal h2 {
  color: var(--color-text-bright);
}

.field {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 0.9rem;
  color: var(--color-text-muted);
}

.field input,
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

.field input:focus,
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

.btn-primary:hover:not(:disabled) {
  background-color: var(--color-pass-strong);
  border-color: var(--color-pass-strong);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
