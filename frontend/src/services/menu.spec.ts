import axios from 'axios'
import { afterEach, describe, expect, it } from 'vitest'

import request from '@/utils/request'
import { loadCurrentMenus } from '@/services/menu'

describe('menu service', () => {
  const originalAdapter = request.defaults.adapter

  afterEach(() => {
    request.defaults.adapter = originalAdapter
  })

  it('uses backend menus when the menus endpoint is available', async () => {
    request.defaults.adapter = async (config) => {
      return {
        data: {
          code: 0,
          message: 'ok',
          data: {
            menus: [
              {
                id: 'workspace',
                name: 'workspace',
                title: '工作台',
                children: [
                  {
                    id: 'dashboard',
                    name: 'dashboard',
                    title: '概览看板',
                    path: '/dashboard',
                  },
                ],
              },
            ],
          },
        },
        status: 200,
        statusText: 'OK',
        headers: {},
        config,
      }
    }

    const menus = await loadCurrentMenus()

    expect(menus).toHaveLength(1)
    expect(menus[0].children?.[0].path).toBe('/dashboard')
  })

  it('normalizes legacy placeholder menu paths to real routes', async () => {
    request.defaults.adapter = async (config) => {
      return {
        data: {
          code: 0,
          message: 'ok',
          data: {
            menus: [
              {
                id: 'system',
                name: 'system',
                title: '系统管理',
                children: [
                  {
                    id: 'roles',
                    name: 'roles',
                    title: '角色管理',
                    path: '/placeholder/roles',
                  },
                ],
              },
            ],
          },
        },
        status: 200,
        statusText: 'OK',
        headers: {},
        config,
      }
    }

    const menus = await loadCurrentMenus()

    expect(menus[0].children?.[0].path).toBe('/system/role')
  })

  it('falls back to local mock menus when the menus endpoint is not implemented yet', async () => {
    request.defaults.adapter = async (config) => {
      throw new axios.AxiosError(
        'Not Found',
        'ERR_BAD_REQUEST',
        config,
        undefined,
        {
          data: { code: 404, message: 'Not Found' },
          status: 404,
          statusText: 'Not Found',
          headers: {},
          config,
        },
      )
    }

    const menus = await loadCurrentMenus()

    expect(menus.length).toBeGreaterThan(0)
    expect(menus[0].id).toBe('dashboard-root')
  })
})
