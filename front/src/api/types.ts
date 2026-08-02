export interface User {
  id: number
  name: string
  predict_death_age: number
  birth_date: number
}

export interface Blueprint {
  id: number
  goal: string
  desc: string
  start_dt: number
  end_dt: number
}

export type BlptCellStatus = 'pass' | 'fail' | 'padding' | 'future'

export interface BlptCell {
  date: number
  status: BlptCellStatus
  is_today: boolean
  note: string | null
}

export interface BlptCellListResponse {
  blpt: Blueprint
  cell_start_dt: number
  cell_end_dt: number
  cells: BlptCell[]
}

export interface Footprint {
  id: number
  blpt_id: number
  day_dt: number
  status: number
  note: string | null
}

export interface LifeFarmCell {
  day_dt: number
  target_rate: number
  actual_rate: number
}

export interface LifeFarmResponse {
  birth_date: number
  predict_death_age: number
  cells: LifeFarmCell[]
}
