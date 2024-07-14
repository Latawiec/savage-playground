import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import GameView from '@/views/GameView.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/game',
    component: GameView,
    children: [
      {
        path: 'proto_chat',
        component: () => import('proto_chat/src/components/ProtoChat.vue')
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
