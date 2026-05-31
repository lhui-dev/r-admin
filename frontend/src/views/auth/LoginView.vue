<script setup lang="ts">
import { Lock, User } from '@element-plus/icons-vue'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage } from 'element-plus'
import { reactive, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const isDev = import.meta.env.DEV

const loading = ref(false)
const formRef = ref<FormInstance>()
const form = reactive({
  username: isDev ? 'admin' : '',
  password: isDev ? 'Admin@123456' : '',
})

const rules: FormRules<typeof form> = {
  username: [{ required: true, message: '请输入账号', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }],
}

async function handleSubmit() {
  const instance = formRef.value
  if (!instance) {
    return
  }

  const valid = await instance.validate().catch(() => false)
  if (!valid) {
    return
  }

  loading.value = true
  try {
    await authStore.login({
      username: form.username.trim(),
      password: form.password,
    })

    ElMessage.success('登录成功，欢迎回来。')

    const redirect = typeof route.query.redirect === 'string'
      ? route.query.redirect
      : '/dashboard'

    await router.replace(redirect)
  }
  catch (error: any) {
    const message = error?.message || error?.response?.data?.message || '登录失败，请稍后重试。'
    ElMessage.error(message)
  }
  finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-view">
    <div class="login-view__shell">
      <section class="login-view__panel">
        <div class="login-view__intro">
          <span class="login-view__eyebrow">r-admin Console</span>
          <h1>登录后台控制台</h1>
          <p>先完成稳定登录态，再进入菜单、权限和 RBAC 业务链路。</p>
        </div>

        <el-form
          ref="formRef"
          :model="form"
          :rules="rules"
          label-position="top"
          class="login-view__form"
          @submit.prevent="handleSubmit"
        >
          <el-form-item
            label="账号"
            prop="username"
          >
            <el-input
              v-model="form.username"
              size="large"
              placeholder="请输入账号"
              :prefix-icon="User"
            />
          </el-form-item>

          <el-form-item
            label="密码"
            prop="password"
          >
            <el-input
              v-model="form.password"
              type="password"
              size="large"
              show-password
              placeholder="请输入密码"
              :prefix-icon="Lock"
              @keyup.enter="handleSubmit"
            />
          </el-form-item>

          <div
            class="login-view__meta"
            :class="{ 'is-production': !isDev }"
          >
            <template v-if="isDev">
              <span>默认调试账号：admin</span>
              <span>密码：Admin@123456</span>
            </template>
            <template v-else>
              <span>请输入已分配的控制台账号与密码。</span>
            </template>
          </div>

          <el-button
            type="primary"
            size="large"
            class="login-view__submit"
            :loading="loading"
            @click="handleSubmit"
          >
            登录并进入控制台
          </el-button>
        </el-form>
      </section>
    </div>
  </div>
</template>

<style scoped>
.login-view {
  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: 32px 20px;
  background:
    radial-gradient(circle at top left, rgba(236, 244, 255, 0.95), transparent 34%),
    radial-gradient(circle at bottom right, rgba(255, 233, 236, 0.86), transparent 32%),
    linear-gradient(160deg, #f6f8fc 0%, #eef3fb 100%);
}

.login-view__shell {
  width: min(100%, 1080px);
}

.login-view__panel {
  display: grid;
  grid-template-columns: minmax(0, 0.95fr) minmax(340px, 420px);
  gap: 36px;
  align-items: center;
  padding: 36px;
  border: 1px solid rgba(201, 212, 228, 0.7);
  border-radius: 28px;
  background: rgba(255, 255, 255, 0.84);
  backdrop-filter: blur(18px);
  box-shadow: 0 24px 80px rgba(15, 23, 42, 0.08);
}

.login-view__intro {
  display: grid;
  gap: 16px;
  max-width: 460px;
}

.login-view__eyebrow {
  display: inline-flex;
  width: fit-content;
  padding: 7px 12px;
  border-radius: 999px;
  background: #e8f2ff;
  color: #1f7aff;
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.login-view__intro h1 {
  margin: 0;
  color: #1d2b3f;
  font-size: clamp(30px, 5vw, 48px);
  line-height: 1.05;
}

.login-view__intro p {
  margin: 0;
  color: #5d6c83;
  font-size: 16px;
  line-height: 1.8;
}

.login-view__form {
  padding: 26px 24px 24px;
  border-radius: 24px;
  background: #ffffff;
  box-shadow: inset 0 0 0 1px rgba(222, 230, 240, 0.8);
}

.login-view__meta {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  margin: 6px 0 18px;
  color: #8090a7;
  font-size: 12px;
  flex-wrap: wrap;
}

.login-view__meta.is-production {
  justify-content: flex-start;
}

.login-view__submit {
  width: 100%;
  height: 46px;
  border-radius: 14px;
  font-weight: 700;
}

@media (max-width: 920px) {
  .login-view__panel {
    grid-template-columns: 1fr;
    padding: 26px;
  }
}

@media (max-width: 640px) {
  .login-view {
    padding: 18px;
  }

  .login-view__panel {
    padding: 20px;
    border-radius: 22px;
  }

  .login-view__form {
    padding: 20px 18px 18px;
    border-radius: 20px;
  }
}
</style>
