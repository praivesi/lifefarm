<template>
  <div class="card" :class="{ archived: isArchived }" @click="$emit('open', blueprint.id)">
    <div class="card-header">
      <span class="flag">{{ blueprint.goal }}</span>
      <div class="hover-actions" @click.stop>
        <button class="icon-btn" title="수정" @click="$emit('edit', blueprint)">✎</button>
        <button class="icon-btn" title="삭제" @click="confirmDelete">🗑</button>
      </div>
    </div>

    <p class="desc">{{ blueprint.desc }}</p>

    <GrassGrid v-if="cells.length" :cells="cells" compact />
    <p v-else class="loading">로딩중...</p>

    <div v-if="isArchived" class="archived-overlay"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import GrassGrid from './GrassGrid.vue'
import { getBlueprintCells } from '../api/footprints'
import type { Blueprint, BlptCell } from '../api/types'

const props = defineProps<{ blueprint: Blueprint }>()
const emit = defineEmits<{
  (e: 'open', id: number): void
  (e: 'edit', blueprint: Blueprint): void
  (e: 'delete', id: number): void
}>()

const cells = ref<BlptCell[]>([])

const isArchived = computed(() => props.blueprint.end_dt * 1000 < Date.now())

onMounted(async () => {
  const res = await getBlueprintCells(props.blueprint.id)
  cells.value = res.cells
})

function confirmDelete() {
  if (window.confirm(`'${props.blueprint.goal}' Blueprint를 삭제할까요? 관련 기록도 함께 삭제됩니다.`)) {
    emit('delete', props.blueprint.id)
  }
}
</script>

<style scoped>
.card {
  position: relative;
  background-color: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: 12px;
  cursor: pointer;
  overflow: hidden;
  transition: border-color 0.15s, background-color 0.15s;
}

.card:hover {
  border-color: var(--color-accent);
  background-color: var(--color-surface-hover);
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 6px;
}

.flag {
  background-color: var(--color-flag);
  color: var(--color-flag-text);
  padding: 4px 10px;
  border-radius: var(--radius-sm);
  font-weight: 600;
  font-size: 0.9rem;
}

.hover-actions {
  display: flex;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.15s;
}

.card:hover .hover-actions {
  opacity: 1;
}

.icon-btn {
  border: 1px solid var(--color-border);
  background-color: var(--color-surface);
  color: var(--color-text);
  border-radius: var(--radius-sm);
  width: 26px;
  height: 26px;
  cursor: pointer;
}

.icon-btn:hover {
  border-color: var(--color-accent);
}

.desc {
  font-size: 0.85rem;
  color: var(--color-text-muted);
  margin-bottom: 8px;
  min-height: 1.2em;
}

.loading {
  font-size: 0.8rem;
  color: var(--color-text-muted);
}

.archived-overlay {
  position: absolute;
  inset: 0;
  pointer-events: none;
  background-image: repeating-linear-gradient(
    45deg,
    rgba(255, 255, 255, 0.04) 0,
    rgba(255, 255, 255, 0.04) 2px,
    transparent 2px,
    transparent 10px
  );
}

.card.archived {
  opacity: 0.7;
}
</style>
