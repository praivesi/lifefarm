import { apiClient } from './client'
import type { Blueprint } from './types'

export interface BlueprintPayload {
  goal: string
  desc: string
  start_dt: number
  end_dt: number
}

export async function listBlueprints(): Promise<Blueprint[]> {
  const res = await apiClient.get<{ blpts: Blueprint[] }>('/front/blpt')
  return res.data.blpts
}

export async function createBlueprint(payload: BlueprintPayload): Promise<Blueprint> {
  const res = await apiClient.post<Blueprint>('/front/blpt', payload)
  return res.data
}

export async function updateBlueprint(id: number, payload: BlueprintPayload): Promise<Blueprint> {
  const res = await apiClient.put<Blueprint>(`/front/blpt/${id}`, payload)
  return res.data
}

export async function deleteBlueprint(id: number): Promise<void> {
  await apiClient.delete(`/front/blpt/${id}`)
}
