import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getLifeFarm } from '../api/lifefarm'
import type { LifeFarmCell } from '../api/types'

export const useLifeFarmStore = defineStore('lifefarm', () => {
  const birthDate = ref<number | null>(null)
  const predictDeathAge = ref<number | null>(null)
  const cells = ref<LifeFarmCell[]>([])

  async function fetchLifeFarm() {
    const res = await getLifeFarm()
    birthDate.value = res.birth_date
    predictDeathAge.value = res.predict_death_age
    cells.value = res.cells
  }

  return { birthDate, predictDeathAge, cells, fetchLifeFarm }
})
