<template>
  <div class="modal-backdrop" @click.self="$emit('close')">
    <div class="modal">
      <h2>내 정보</h2>

      <label class="field">
        <span>이름</span>
        <input v-model="form.name" type="text" />
      </label>

      <label class="field">
        <span>태어난 날</span>
        <input v-model="birthDateInput" type="date" />
      </label>

      <label class="field">
        <span>예상 사망 나이</span>
        <input v-model.number="form.predict_death_age" type="number" min="1" max="150" />
      </label>

      <div class="modal-actions">
        <button class="btn" @click="$emit('close')">취소</button>
        <button class="btn btn-primary" @click="save">저장</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { User } from '../api/types'

const props = defineProps<{ user: User | null }>()
const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', payload: Omit<User, 'id'>): void
}>()

const form = ref({
  name: props.user?.name ?? '',
  predict_death_age: props.user?.predict_death_age ?? 80
})

const birthDateInput = ref(
  props.user ? new Date(props.user.birth_date * 1000).toISOString().slice(0, 10) : ''
)

const birthDateTimestamp = computed(() => {
  if (!birthDateInput.value) return 0
  return Math.floor(new Date(`${birthDateInput.value}T00:00:00+09:00`).getTime() / 1000)
})

function save() {
  emit('save', {
    name: form.value.name,
    predict_death_age: form.value.predict_death_age,
    birth_date: birthDateTimestamp.value
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
  width: 320px;
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

.field input {
  padding: 6px 8px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background-color: var(--color-bg);
  font-size: 1rem;
  color: var(--color-text);
}

.field input:focus {
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
