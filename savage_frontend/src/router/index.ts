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
      },
      {
        path: 'ffxiv_toolkit_debug',
        component: () => import('ffxiv-toolkit-debug-frontend').then(module => module.FfxivToolkitDebug)
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
