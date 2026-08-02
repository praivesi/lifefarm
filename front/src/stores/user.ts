import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getUser, updateUser } from '../api/user'
import type { User } from '../api/types'

export const useUserStore = defineStore('user', () => {
  const user = ref<User | null>(null)

  async function fetchUser() {
    user.value = await getUser()
  }

  async function saveUser(payload: Omit<User, 'id'>) {
    user.value = await updateUser(payload)
  }

  return { user, fetchUser, saveUser }
})
