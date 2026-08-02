<template>
  <div class="list-view">
    <h1>내 Blueprint</h1>

    <p v-if="!blueprintStore.list.length" class="empty">
      아직 Blueprint가 없어요. 상단의 ＋ 버튼으로 첫 목표를 심어보세요.
    </p>

    <div class="grid">
      <BlueprintCard
        v-for="blpt in blueprintStore.list"
        :key="blpt.id"
        :blueprint="blpt"
        @open="goToDetail"
        @edit="startEdit"
        @delete="handleDelete"
      />
    </div>

    <BlueprintFormModal
      v-if="editingBlueprint"
      :blueprint="editingBlueprint"
      @close="editingBlueprint = null"
      @save="handleUpdate"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import BlueprintCard from '../components/BlueprintCard.vue'
import BlueprintFormModal from '../components/BlueprintFormModal.vue'
import { useBlueprintStore } from '../stores/blueprints'
import type { Blueprint } from '../api/types'
import type { BlueprintPayload } from '../api/blueprints'

const router = useRouter()
const blueprintStore = useBlueprintStore()

const editingBlueprint = ref<Blueprint | null>(null)

onMounted(() => {
  blueprintStore.fetchList()
})

function goToDetail(id: number) {
  router.push(`/blueprints/${id}`)
}

function startEdit(blueprint: Blueprint) {
  editingBlueprint.value = blueprint
}

async function handleUpdate(payload: BlueprintPayload) {
  if (!editingBlueprint.value) return
  await blueprintStore.update(editingBlueprint.value.id, payload)
  editingBlueprint.value = null
}

async function handleDelete(id: number) {
  await blueprintStore.remove(id)
}
</script>

<style scoped>
.list-view h1 {
  margin-bottom: 16px;
  color: var(--color-text-bright);
}

.empty {
  color: var(--color-text-muted);
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 16px;
}
</style>
