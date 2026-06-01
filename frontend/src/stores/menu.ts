import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import { loadCurrentMenus, type LoadCurrentMenusOptions } from '@/services/menu'
import type { AppMenuItem } from '@/types/menu'

export const useMenuStore = defineStore('menu', () => {
  const menuTree = ref<AppMenuItem[]>([])
  const initialized = ref(false)

  const visibleMenuTree = computed(() => menuTree.value)

  async function initMenus(options: LoadCurrentMenusOptions = {}) {
    const sourceMenus = await loadCurrentMenus(options)
    menuTree.value = filterMenuTree(sourceMenus, options)
    initialized.value = true
  }

  function setMenus(menus: AppMenuItem[]) {
    menuTree.value = menus
    initialized.value = true
  }

  function resetMenus() {
    menuTree.value = []
    initialized.value = false
  }

  return {
    menuTree,
    initialized,
    visibleMenuTree,
    initMenus,
    setMenus,
    resetMenus,
  }
})

function filterMenuTree(menus: AppMenuItem[], options: LoadCurrentMenusOptions) {
  return menus
    .map((item) => normalizeMenuItem(item, options))
    .filter((item): item is AppMenuItem => Boolean(item))
}

function normalizeMenuItem(item: AppMenuItem, options: LoadCurrentMenusOptions): AppMenuItem | null {
  if (item.hidden) {
    return null
  }

  const visibleChildren = item.children
    ?.map((child) => normalizeMenuItem(child, options))
    .filter((child): child is AppMenuItem => Boolean(child))

  const hasVisibleChildren = Boolean(visibleChildren?.length)
  const canDisplaySelf = hasMenuAccess(item, options)

  if (!canDisplaySelf && !hasVisibleChildren) {
    return null
  }

  return {
    ...item,
    children: visibleChildren,
  }
}

function hasMenuAccess(item: AppMenuItem, options: LoadCurrentMenusOptions) {
  if (!item.permission) {
    return true
  }

  if (options.isSuperAdmin) {
    return true
  }

  return options.permissions?.includes(item.permission) ?? false
}
