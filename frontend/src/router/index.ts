import {
  createRouter,
  createWebHistory,
  type RouteRecordRaw,
  type RouterHistory,
} from 'vue-router'

import { pinia } from '@/stores'
import { useAuthStore } from '@/stores/auth'
import AppLayout from '@/layouts/AppLayout.vue'
import LoginView from '@/views/auth/LoginView.vue'
import DashboardView from '@/views/dashboard/DashboardView.vue'
import NotFoundView from '@/views/not-found/NotFoundView.vue'
import ProfileView from '@/views/profile/ProfileView.vue'
import SystemView from '@/views/system/SystemView.vue'

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: AppLayout,
    redirect: '/dashboard',
    meta: {
      requiresAuth: true,
    },
    children: [
      {
        path: 'dashboard',
        name: 'dashboard',
        component: DashboardView,
      },
      {
        path: 'system',
        name: 'system',
        component: SystemView,
      },
      {
        path: 'profile',
        name: 'profile',
        component: ProfileView,
      },
    ],
  },
  {
    path: '/login',
    name: 'login',
    component: LoginView,
    meta: {
      guestOnly: true,
    },
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'not-found',
    component: NotFoundView,
  },
]

export function createAppRouter(history: RouterHistory = createWebHistory()) {
  const router = createRouter({
    history,
    routes,
  })

  router.beforeEach(async (to) => {
    const authStore = useAuthStore(pinia)

    if (to.meta.requiresAuth) {
      if (!authStore.hasToken) {
        return {
          path: '/login',
          query: { redirect: to.fullPath },
        }
      }

      if (!authStore.isAuthenticated) {
        try {
          // Refreshes usually land here: token exists, but the in-memory user
          // state still needs to be restored from /me before entering the app.
          await authStore.bootstrap()
        }
        catch {
          return {
            path: '/login',
            query: { redirect: to.fullPath },
          }
        }
      }
    }

    if (to.meta.guestOnly) {
      if (!authStore.hasToken) {
        return true
      }

      if (!authStore.isAuthenticated) {
        try {
          await authStore.bootstrap()
        }
        catch {
          // Let the login page render if the persisted token can no longer be
          // restored, instead of bouncing the user between redirects.
          return true
        }
      }

      return '/dashboard'
    }

    return true
  })

  return router
}

const router = createAppRouter()

export default router
