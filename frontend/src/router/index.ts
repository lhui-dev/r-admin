import {
  createRouter,
  createWebHistory,
  type RouteRecordRaw,
  type RouterHistory,
} from 'vue-router'

import { pinia } from '@/stores'
import { useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'
import AppLayout from '@/layouts/AppLayout.vue'
import LoginView from '@/views/auth/LoginView.vue'
import DashboardView from '@/views/dashboard/DashboardView.vue'
import NotFoundView from '@/views/not-found/NotFoundView.vue'
import ConstructionView from '@/views/placeholder/ConstructionView.vue'
import ProfileView from '@/views/profile/ProfileView.vue'
import SystemView from '@/views/system/SystemView.vue'
import UserManagementView from '@/views/system/user/UserManagementView.vue'

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
        path: 'system/user',
        name: 'system-user',
        component: UserManagementView,
      },
      {
        path: 'system/role',
        name: 'system-role',
        component: ConstructionView,
        meta: {
          featureKey: 'roles',
        },
      },
      {
        path: 'system/menu',
        name: 'system-menu',
        component: ConstructionView,
        meta: {
          featureKey: 'menus',
        },
      },
      {
        path: 'system/dept',
        name: 'system-dept',
        component: ConstructionView,
        meta: {
          featureKey: 'departments',
        },
      },
      {
        path: 'system/post',
        name: 'system-post',
        component: ConstructionView,
        meta: {
          featureKey: 'posts',
        },
      },
      {
        path: 'system/dict',
        name: 'system-dict',
        component: ConstructionView,
        meta: {
          featureKey: 'dictionaries',
        },
      },
      {
        path: 'system/config',
        name: 'system-config',
        component: ConstructionView,
        meta: {
          featureKey: 'configs',
        },
      },
      {
        path: 'audit/login-log',
        name: 'audit-login-log',
        component: ConstructionView,
        meta: {
          featureKey: 'login-logs',
        },
      },
      {
        path: 'audit/operation-log',
        name: 'audit-operation-log',
        component: ConstructionView,
        meta: {
          featureKey: 'operation-logs',
        },
      },
      {
        path: 'profile',
        name: 'profile',
        component: ProfileView,
      },
      {
        path: 'placeholder/users',
        redirect: '/system/user',
      },
      {
        path: 'placeholder/:feature',
        name: 'placeholder',
        component: ConstructionView,
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
    const menuStore = useMenuStore(pinia)

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

      await ensureMenusReady(authStore, menuStore)
    }

    if (to.meta.guestOnly) {
      if (!authStore.hasToken) {
        return true
      }

      if (!authStore.isAuthenticated) {
        try {
          await authStore.bootstrap()
          await ensureMenusReady(authStore, menuStore)
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

async function ensureMenusReady(
  authStore: ReturnType<typeof useAuthStore>,
  menuStore: ReturnType<typeof useMenuStore>,
) {
  if (!authStore.currentUser || menuStore.initialized) {
    return
  }

  await menuStore.initMenus({
    permissions: authStore.permissions,
    isSuperAdmin: authStore.currentUser.is_super_admin,
  })
}
