<template>
  <div class="lifefarm-view">
    <h1>인생 농장</h1>

    <div class="toggle">
      <button class="toggle-btn" :class="{ active: mode === 'target' }" @click="mode = 'target'">
        목표치
      </button>
      <button class="toggle-btn" :class="{ active: mode === 'actual' }" @click="mode = 'actual'">
        실달성치
      </button>
    </div>

    <LifeGrid
      v-if="lifeFarmStore.cells.length && lifeFarmStore.birthDate"
      :cells="lifeFarmStore.cells"
      :birth-date="lifeFarmStore.birthDate"
      :mode="mode"
    />
    <p v-else class="loading">로딩중...</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import LifeGrid from '../components/LifeGrid.vue'
import { useLifeFarmStore } from '../stores/lifefarm'

const lifeFarmStore = useLifeFarmStore()
const mode = ref<'target' | 'actual'>('actual')

onMounted(() => {
  lifeFarmStore.fetchLifeFarm()
})
</script>

<style scoped>
.lifefarm-view h1 {
  margin-bottom: 16px;
  color: var(--color-text-bright);
}

.toggle {
  display: flex;
  gap: 8px;
  margin-bottom: 20px;
}

.toggle-btn {
  padding: 6px 14px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border);
  background-color: var(--color-surface);
  color: var(--color-text);
  cursor: pointer;
}

.toggle-btn:hover {
  border-color: var(--color-accent);
}

.toggle-btn.active {
  background-color: var(--color-border-muted);
  border-color: var(--color-accent);
  color: var(--color-text-bright);
  font-weight: 600;
}

.loading {
  color: var(--color-text-muted);
}
</style>
