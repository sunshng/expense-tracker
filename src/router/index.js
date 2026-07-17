import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/HomeView.vue'),
    meta: { title: '账单列表' }
  },
  {
    path: '/add',
    name: 'AddRecord',
    component: () => import('../views/AddRecord.vue'),
    meta: { title: '记一笔' }
  },
  {
    path: '/statistics',
    name: 'Statistics',
    component: () => import('../views/Statistics.vue'),
    meta: { title: '统计分析' }
  }
]

// 使用 hash history，兼容桌面端（Tauri）和手机端（PWA）
const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
