import { apiClient } from './client'
import type { BlptCellListResponse, Footprint } from './types'

export interface CertifyCellPayload {
  day_dt: number
  status: number
  note: string | null
}

export async function getBlueprintCells(blptId: number): Promise<BlptCellListResponse> {
  const res = await apiClient.get<BlptCellListResponse>(`/front/blpt/${blptId}/cell`)
  return res.data
}

export async function certifyCell(blptId: number, payload: CertifyCellPayload): Promise<Footprint> {
  const res = await apiClient.post<Footprint>(`/front/blpt/${blptId}/cell`, payload)
  return res.data
}
