import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getBlueprintCells, certifyCell, type CertifyCellPayload } from '../api/footprints'
import type { Blueprint, BlptCell } from '../api/types'

export const useBlueprintDetailStore = defineStore('blueprintDetail', () => {
  const current = ref<Blueprint | null>(null)
  const cells = ref<BlptCell[]>([])

  async function fetchCells(blptId: number) {
    const res = await getBlueprintCells(blptId)
    current.value = res.blpt
    cells.value = res.cells
  }

  async function upsertCell(blptId: number, payload: CertifyCellPayload) {
    await certifyCell(blptId, payload)
    await fetchCells(blptId)
  }

  return { current, cells, fetchCells, upsertCell }
})
