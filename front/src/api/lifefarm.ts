import { apiClient } from './client'
import type { LifeFarmResponse } from './types'

export async function getLifeFarm(): Promise<LifeFarmResponse> {
  const res = await apiClient.get<LifeFarmResponse>('/front/lifefarm')
  return res.data
}
