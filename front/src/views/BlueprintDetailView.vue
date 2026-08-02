<template>
  <div class="detail-view" v-if="detailStore.current">
    <RouterLink to="/" class="back-link">← 목록으로</RouterLink>

    <div class="header">
      <span class="flag">{{ detailStore.current.goal }}</span>
      <p class="desc">{{ detailStore.current.desc }}</p>
      <p class="period">
        {{ formatDate(detailStore.current.start_dt) }} ~ {{ formatDate(detailStore.current.end_dt) }}
      </p>
    </div>

    <GrassGrid :cells="detailStore.cells" @cell-click="handleCellClick" />

    <CertifyCellModal
      v-if="selectedCell"
      :cell="selectedCell"
      @close="selectedCell = null"
      @save="handleSave"
    />
  </div>
  <p v-else class="loading">로딩중...</p>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import GrassGrid from '../components/GrassGrid.vue'
import CertifyCellModal from '../components/CertifyCellModal.vue'
import { useBlueprintDetailStore } from '../stores/blueprintDetail'
import type { BlptCell } from '../api/types'

const props = defineProps<{ id: string }>()
const detailStore = useBlueprintDetailStore()

const selectedCell = ref<BlptCell | null>(null)

onMounted(() => {
  detailStore.fetchCells(Number(props.id))
})

function handleCellClick(cell: BlptCell) {
  if (cell.status === 'padding') return
  selectedCell.value = cell
}

async function handleSave(payload: { status: number; note: string | null }) {
  if (!selectedCell.value) return

  await detailStore.upsertCell(Number(props.id), {
    day_dt: selectedCell.value.date,
    status: payload.status,
    note: payload.note
  })

  selectedCell.value = null
}

function formatDate(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleDateString('ko-KR')
}
</script>

<style scoped>
.back-link {
  display: inline-block;
  margin-bottom: 16px;
  color: var(--color-text-muted);
  text-decoration: none;
}

.back-link:hover {
  color: var(--color-accent);
}

.header {
  margin-bottom: 16px;
}

.flag {
  display: inline-block;
  background-color: var(--color-flag);
  color: var(--color-flag-text);
  padding: 6px 14px;
  border-radius: var(--radius-sm);
  font-weight: 600;
  margin-bottom: 8px;
}

.desc {
  color: var(--color-text-muted);
  margin: 6px 0;
}

.period {
  font-size: 0.85rem;
  color: var(--color-text-muted);
}

.loading {
  color: var(--color-text-muted);
}
</style>
