<template>
  <header class="app-header">
    <RouterLink to="/" class="brand">🌱 LifeFarm</RouterLink>

    <nav class="nav">
      <RouterLink to="/" class="nav-link">Blueprint</RouterLink>
      <RouterLink to="/lifefarm" class="nav-link">인생 농장</RouterLink>
    </nav>

    <div class="actions">
      <button class="icon-btn" title="Blueprint 추가" @click="$emit('add-blueprint')">＋</button>
      <button class="icon-btn" title="설정" @click="$emit('open-settings')">⚙</button>
    </div>
  </header>
</template>

<script setup lang="ts">
defineEmits<{
  (e: 'add-blueprint'): void
  (e: 'open-settings'): void
}>()
</script>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  gap: 24px;
  padding: 12px 20px;
  background-color: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.brand {
  font-weight: 600;
  font-size: 1.1rem;
  color: var(--color-text-bright);
  text-decoration: none;
}

.nav {
  display: flex;
  gap: 4px;
  flex: 1;
}

.nav-link {
  color: var(--color-text-muted);
  text-decoration: none;
  padding: 6px 12px;
  border-radius: var(--radius-sm);
  font-size: 0.9rem;
  transition: background-color 0.1s, color 0.1s;
}

.nav-link:hover {
  color: var(--color-text-bright);
  background-color: var(--color-surface-hover);
}

.nav-link.router-link-active {
  color: var(--color-text-bright);
  font-weight: 600;
  background-color: var(--color-border-muted);
}

.actions {
  display: flex;
  gap: 8px;
}

.icon-btn {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: 1px solid var(--color-border);
  background-color: var(--color-surface);
  color: var(--color-text);
  cursor: pointer;
  font-size: 1rem;
  line-height: 1;
  transition: background-color 0.1s, border-color 0.1s;
}

.icon-btn:hover {
  background-color: var(--color-surface-hover);
  border-color: var(--color-accent);
}
</style>
