import axios from 'axios'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'

import { getToken, setToken } from '@/utils/auth'
import request, { registerUnauthorizedHandler } from '@/utils/request'

describe('auth request handling', () => {
  const originalAdapter = request.defaults.adapter

  beforeEach(() => {
    window.localStorage.clear()
    registerUnauthorizedHandler(() => {})
  })

  afterEach(() => {
    request.defaults.adapter = originalAdapter
    registerUnauthorizedHandler(() => {})
  })

  it('injects the bearer token into outgoing requests', async () => {
    setToken('smoke-token')

    let authorizationHeader = ''
    request.defaults.adapter = async (config) => {
      authorizationHeader = String(config.headers.Authorization ?? '')

      return {
        data: { ok: true },
        status: 200,
        statusText: 'OK',
        headers: {},
        config,
      }
    }

    const response = await request.get('/auth/me')

    expect(response).toEqual({ ok: true })
    expect(authorizationHeader).toBe('Bearer smoke-token')
  })

  it('clears the stored token and notifies the unauthorized handler on 401', async () => {
    setToken('expired-token')

    let unauthorizedHandled = false
    registerUnauthorizedHandler(() => {
      unauthorizedHandled = true
    })

    request.defaults.adapter = async (config) => {
      throw new axios.AxiosError(
        'Unauthorized',
        'ERR_BAD_REQUEST',
        config,
        undefined,
        {
          data: { code: 401, message: 'unauthorized' },
          status: 401,
          statusText: 'Unauthorized',
          headers: {},
          config,
        },
      )
    }

    await expect(request.get('/auth/me')).rejects.toEqual({ code: 401, message: 'unauthorized' })
    expect(getToken()).toBe('')
    expect(unauthorizedHandled).toBe(true)
  })
})
