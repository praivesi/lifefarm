<template>
  <div class="app-shell">
    <AppHeader @add-blueprint="showBlueprintForm = true" @open-settings="showUserSettings = true" />

    <main class="app-content">
      <RouterView />
    </main>

    <UserSettingsModal
      v-if="showUserSettings"
      :user="userStore.user"
      @close="showUserSettings = false"
      @save="handleSaveUser"
    />

    <BlueprintFormModal
      v-if="showBlueprintForm"
      :blueprint="null"
      @close="showBlueprintForm = false"
      @save="handleCreateBlueprint"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AppHeader from './components/AppHeader.vue'
import UserSettingsModal from './components/UserSettingsModal.vue'
import BlueprintFormModal from './components/BlueprintFormModal.vue'
import { useUserStore } from './stores/user'
import { useBlueprintStore } from './stores/blueprints'
import type { User } from './api/types'
import type { BlueprintPayload } from './api/blueprints'

const userStore = useUserStore()
const blueprintStore = useBlueprintStore()

const showUserSettings = ref(false)
const showBlueprintForm = ref(false)

onMounted(() => {
  userStore.fetchUser()
})

async function handleSaveUser(payload: Omit<User, 'id'>) {
  await userStore.saveUser(payload)
  showUserSettings.value = false
}

async function handleCreateBlueprint(payload: BlueprintPayload) {
  await blueprintStore.create(payload)
  showBlueprintForm.value = false
}
</script>

<style>
body {
  background-color: var(--color-bg);
  color: var(--color-text);
}

.app-shell {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.app-content {
  flex: 1;
  padding: 24px;
}
</style>
