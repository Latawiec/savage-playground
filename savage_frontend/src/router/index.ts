import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import GameView from '../views/GameView.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/game',
    component: GameView,
    children: [
      {
        path: 'proto_chat',
        component: () => import('proto-chat-frontend').then(module => module.ProtoChat)
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
