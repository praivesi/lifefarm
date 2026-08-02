<template>
  <div class="lifegrid">
    <div class="row" v-for="(row, rowIdx) in rows" :key="rowIdx">
      <span class="year-label">{{ row.startYear }} - {{ row.endYear }}</span>
      <div class="week-cells">
        <div
          v-for="(week, weekIdx) in row.weeks"
          :key="weekIdx"
          class="week-cell"
          :class="{ 'is-current': week.isCurrent }"
          :style="{ backgroundColor: colorFor(week.rate) }"
          :title="`${week.rate !== null ? Math.round(week.rate * 100) + '%' : '기록 없음'}`"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { LifeFarmCell } from '../api/types'

const props = defineProps<{
  cells: LifeFarmCell[]
  birthDate: number
  mode: 'target' | 'actual'
}>()

const WEEKS_PER_ROW_YEARS = 2
const DAY_SECONDS = 86400

interface WeekBucket {
  rate: number | null
  isCurrent: boolean
}

interface Row {
  startYear: number
  endYear: number
  weeks: WeekBucket[]
}

const rows = computed<Row[]>(() => {
  if (!props.cells.length) return []

  const birthYear = new Date(props.birthDate * 1000).getFullYear()
  const today = Math.floor(Date.now() / 1000)
  const todayWeekIdx = Math.floor((today - props.birthDate) / (DAY_SECONDS * 7))

  // bucket daily cells into weeks
  const weekBuckets: { targetSum: number; actualSum: number; count: number }[] = []
  for (const cell of props.cells) {
    const dayIdx = Math.floor((cell.day_dt - props.birthDate) / DAY_SECONDS)
    const weekIdx = Math.floor(dayIdx / 7)
    if (!weekBuckets[weekIdx]) weekBuckets[weekIdx] = { targetSum: 0, actualSum: 0, count: 0 }
    weekBuckets[weekIdx].targetSum += cell.target_rate
    weekBuckets[weekIdx].actualSum += cell.actual_rate
    weekBuckets[weekIdx].count += 1
  }

  const weeksPerRow = WEEKS_PER_ROW_YEARS * 52
  const totalWeeks = weekBuckets.length

  const result: Row[] = []
  for (let rowStart = 0; rowStart < totalWeeks; rowStart += weeksPerRow) {
    const weeks: WeekBucket[] = []
    for (let w = rowStart; w < rowStart + weeksPerRow; w++) {
      const bucket = weekBuckets[w]
      const rate = bucket
        ? (props.mode === 'target' ? bucket.targetSum : bucket.actualSum) / bucket.count
        : null
      weeks.push({ rate, isCurrent: w === todayWeekIdx })
    }
    result.push({
      startYear: birthYear + rowStart / 52,
      endYear: birthYear + (rowStart + weeksPerRow) / 52 - 1,
      weeks
    })
  }

  return result
})

function colorFor(rate: number | null): string {
  if (rate === null) return 'var(--color-empty)'
  const color = props.mode === 'target' ? '38, 166, 65' : '88, 166, 255'
  return `rgba(${color}, ${Math.max(rate, 0.12)})`
}
</script>

<style scoped>
.lifegrid {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.year-label {
  width: 90px;
  font-size: 0.75rem;
  color: var(--color-text-muted);
  flex-shrink: 0;
}

.week-cells {
  display: flex;
  flex-wrap: wrap;
  gap: 1px;
}

.week-cell {
  width: 6px;
  height: 6px;
  border: 1px solid var(--color-border-muted);
  border-radius: 1px;
}

.week-cell.is-current {
  outline: 2px solid var(--color-today);
  outline-offset: 1px;
}
</style>
