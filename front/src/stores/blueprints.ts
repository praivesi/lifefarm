import { defineStore } from 'pinia'
import { ref } from 'vue'
import {
  listBlueprints,
  createBlueprint,
  updateBlueprint,
  deleteBlueprint,
  type BlueprintPayload
} from '../api/blueprints'
import type { Blueprint } from '../api/types'

export const useBlueprintStore = defineStore('blueprints', () => {
  const list = ref<Blueprint[]>([])

  async function fetchList() {
    list.value = await listBlueprints()
  }

  async function create(payload: BlueprintPayload) {
    const created = await createBlueprint(payload)
    list.value.push(created)
    return created
  }

  async function update(id: number, payload: BlueprintPayload) {
    const updated = await updateBlueprint(id, payload)
    const idx = list.value.findIndex((b) => b.id === id)
    if (idx !== -1) list.value[idx] = updated
    return updated
  }

  async function remove(id: number) {
    await deleteBlueprint(id)
    list.value = list.value.filter((b) => b.id !== id)
  }

  return { list, fetchList, create, update, remove }
})
