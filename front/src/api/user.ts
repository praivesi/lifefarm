import { apiClient } from './client'
import type { User } from './types'

export async function getUser(): Promise<User> {
  const res = await apiClient.get<User>('/front/user')
  return res.data
}

export async function updateUser(payload: Omit<User, 'id'>): Promise<User> {
  const res = await apiClient.put<User>('/front/user', payload)
  return res.data
}
