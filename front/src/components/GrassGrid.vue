<template>
  <div class="grass-grid" :class="{ compact }">
    <div v-if="!compact" class="weekday-col">
      <span v-for="wd in weekdays" :key="wd" class="weekday-label">{{ wd }}</span>
    </div>

    <div class="weeks">
      <div v-for="(week, weekIdx) in weeks" :key="weekIdx" class="week-col">
        <div
          v-for="(cell, dayIdx) in week"
          :key="dayIdx"
          class="cell"
          :class="[`cell-${cell.status}`, { 'cell-today': cell.is_today }]"
          :title="cellTitle(cell)"
          @click="!compact && $emit('cell-click', cell)"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { BlptCell } from '../api/types'

const props = withDefaults(
  defineProps<{
    cells: BlptCell[]
    compact?: boolean
  }>(),
  { compact: false }
)

defineEmits<{
  (e: 'cell-click', cell: BlptCell): void
}>()

const weekdays = ['S', 'M', 'T', 'W', 'T', 'F', 'S']

const weeks = computed(() => {
  const chunks: BlptCell[][] = []
  for (let i = 0; i < props.cells.length; i += 7) {
    chunks.push(props.cells.slice(i, i + 7))
  }
  return chunks
})

function cellTitle(cell: BlptCell): string {
  if (cell.status === 'padding') return ''
  const date = new Date(cell.date * 1000).toLocaleDateString('ko-KR')
  const statusLabel = { pass: '완료', fail: '실패', future: '미정', padding: '' }[cell.status]
  return cell.note ? `${date} - ${statusLabel}: ${cell.note}` : `${date} - ${statusLabel}`
}
</script>

<style scoped>
.grass-grid {
  display: flex;
  gap: 4px;
  padding: 4px;
}

.weekday-col {
  display: flex;
  flex-direction: column;
  gap: var(--grass-cell-gap);
  margin-right: 4px;
}

.weekday-label {
  width: var(--grass-cell-size);
  height: var(--grass-cell-size);
  font-size: 9px;
  color: var(--color-text-muted);
  display: flex;
  align-items: center;
  justify-content: center;
}

.weeks {
  display: flex;
  gap: var(--grass-cell-gap);
}

.week-col {
  display: flex;
  flex-direction: column;
  gap: var(--grass-cell-gap);
}

.cell {
  width: var(--grass-cell-size);
  height: var(--grass-cell-size);
  border-radius: 3px;
  cursor: pointer;
  box-sizing: border-box;
  transition: outline-color 0.1s;
}

.cell:hover {
  outline: 1px solid var(--color-accent);
  outline-offset: -1px;
}

.grass-grid.compact .cell {
  cursor: default;
}

.grass-grid.compact .cell:hover {
  outline: none;
}

.cell-padding {
  border: 1px dashed var(--color-border-muted);
  background: transparent;
}

.cell-future {
  background-color: var(--color-empty);
  border: 1px solid var(--color-border);
}

.cell-pass {
  background-color: var(--color-pass);
}

.cell-fail {
  background-color: var(--color-fail);
}

.cell-today {
  outline: 2px solid var(--color-today);
  outline-offset: 1px;
}
</style>
