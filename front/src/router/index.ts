import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'blueprint-list',
      component: () => import('../views/BlueprintListView.vue')
    },
    {
      path: '/blueprints/:id',
      name: 'blueprint-detail',
      component: () => import('../views/BlueprintDetailView.vue'),
      props: true
    },
    {
      path: '/lifefarm',
      name: 'lifefarm',
      component: () => import('../views/LifeFarmView.vue')
    }
  ]
})

export default router
