<script setup lang="ts">
import {
  CollectionTag,
  Connection,
  CreditCard,
  DataAnalysis,
  Histogram,
  Search,
  Timer,
  User,
} from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { computed, onMounted, ref } from 'vue'

import { fetchHealth, type HealthResponse } from '@/api/system'

type AnalyticsTab = 'consumption' | 'trend' | 'count-distribution' | 'count-ranking'
type ChartKind = 'bar' | 'ranking'
type ChartUnit = 'currency' | 'count'
type MetricItem = {
  icon: unknown
  color: string
  label: string
  value: string
  action?: string
}
type DashboardCard = {
  title: string
  icon: unknown
  metrics: MetricItem[]
  sparkline?: {
    first: { color: string; values: number[] }
    second: { color: string; values: number[] }
  }
}
type ChartConfig = {
  title: string
  total: string
  values: number[]
  labels: string[]
  kind: ChartKind
  unit: ChartUnit
}
type ProgressItem = {
  label: string
  progress: number
  status: string
}

const health = ref<HealthResponse['data'] | null>(null)
const loading = ref(false)
const granularity = ref('hour')
const activeTab = ref<AnalyticsTab>('consumption')

const endTime = ref(new Date())
const startTime = ref(new Date(Date.now() - 24 * 60 * 60 * 1000))

const roleDistributionValues = [38, 24, 18, 16, 12, 8]
const loginTrendValues = [42, 57, 49, 68, 63, 79]
const permissionDistributionValues = [56, 34, 22, 18, 9, 7]
const rankingValues = [36, 31, 28, 22, 18, 15]
const chartPlotHeight = 184

const analyticsTabs = [
  { key: 'consumption' as const, label: '角色分布' },
  { key: 'trend' as const, label: '登录趋势' },
  { key: 'count-distribution' as const, label: '权限类型分布' },
  { key: 'count-ranking' as const, label: '高频访问模块' },
]

const greeting = computed(() => {
  const hour = new Date().getHours()

  if (hour < 6) {
    return '凌晨好'
  }

  if (hour < 12) {
    return '上午好'
  }

  if (hour < 18) {
    return '下午好'
  }

  return '晚上好'
})

const dashboardCards = computed<DashboardCard[]>(() => [
  {
    title: '用户与组织',
    icon: User,
    metrics: [
      {
        icon: User,
        color: '#64a9ff',
        label: '用户总数',
        value: '128',
      },
      {
        icon: CreditCard,
        color: '#c06be7',
        label: '部门数量',
        value: '9',
      },
    ],
  },
  {
    title: '权限资产',
    icon: CollectionTag,
    metrics: [
      {
        icon: Connection,
        color: '#ffd94d',
        label: '角色总数',
        value: '16',
      },
      {
        icon: Histogram,
        color: '#ff6b9d',
        label: '权限点数',
        value: '142',
      },
    ],
    sparkline: {
      first: { color: '#f59e0b', values: [10, 11, 12, 12, 13, 14, 15, 16] },
      second: { color: '#ec4899', values: [84, 91, 98, 106, 118, 124, 133, 142] },
    },
  },
  {
    title: '审计与安全',
    icon: Timer,
    metrics: [
      {
        icon: Timer,
        color: '#8591ff',
        label: '今日登录次数',
        value: '87',
      },
      {
        icon: DataAnalysis,
        color: '#ffb64d',
        label: '待审异常记录',
        value: '3',
      },
    ],
    sparkline: {
      first: { color: '#6366f1', values: [32, 38, 46, 54, 61, 66, 73, 87] },
      second: { color: '#f97316', values: [7, 6, 6, 5, 4, 4, 3, 3] },
    },
  },
])

const activeChart = computed<ChartConfig>(() => {
  const chartMap: Record<AnalyticsTab, ChartConfig> = {
    consumption: {
      title: '角色成员分布',
      total: '116 人',
      values: roleDistributionValues,
      labels: ['超级管理员', '系统管理员', '审计员', '部门主管', '普通成员', '访客'],
      kind: 'bar',
      unit: 'count',
    },
    trend: {
      title: '近 6 个时间段登录趋势',
      total: '358 次',
      values: loginTrendValues,
      labels: ['周一', '周二', '周三', '周四', '周五', '周六'],
      kind: 'bar',
      unit: 'count',
    },
    'count-distribution': {
      title: '权限资源类型分布',
      total: '146 项',
      values: permissionDistributionValues,
      labels: ['菜单权限', '页面权限', '按钮权限', '接口权限', '数据权限', '字典权限'],
      kind: 'bar',
      unit: 'count',
    },
    'count-ranking': {
      title: '高频访问模块排行',
      total: 'Top 6',
      values: rankingValues,
      labels: ['用户管理', '角色管理', '权限点管理', '菜单管理', '部门管理', '审计日志'],
      kind: 'ranking',
      unit: 'count',
    },
  }

  return chartMap[activeTab.value]
})

const maxChartValue = computed(() => Math.max(...activeChart.value.values))
const chartAxisLabels = computed(() => {
  if (activeChart.value.kind !== 'bar') {
    return []
  }

  const steps = [1, 0.75, 0.5, 0.25]
  return steps.map((step) => formatChartValue(maxChartValue.value * step, activeChart.value.unit))
})
const projectProgressItems = computed<ProgressItem[]>(() => [
  {
    label: '需求与数据库设计',
    progress: 100,
    status: '已完成',
  },
  {
    label: '后端基础能力',
    progress: health.value?.status === 'ok' ? 78 : 70,
    status: health.value?.status === 'ok' ? '健康检查已接通' : '基础接口建设中',
  },
  {
    label: '前端 RBAC 控制台原型',
    progress: 72,
    status: '布局与看板已成型',
  },
  {
    label: '用户/角色/权限联调',
    progress: 28,
    status: '待进入联调阶段',
  },
])

const completedProgressCount = computed(() => projectProgressItems.value.filter((item) => item.progress >= 100).length)
const progressSummary = computed(() => {
  const backendStatus = health.value?.status === 'ok' ? '正常' : '待确认'
  const databaseStatus = health.value?.database ?? 'unknown'

  return {
    phase: '前端控制台细化阶段',
    backendStatus,
    databaseStatus,
    service: health.value?.service ?? 'backend',
  }
})

function buildSparkline(values: number[]) {
  const width = 130
  const height = 46
  const max = Math.max(...values)
  const min = Math.min(...values)
  const distance = max - min || 1

  return values
    .map((value, index) => {
      const x = (index / (values.length - 1)) * width
      const y = height - ((value - min) / distance) * (height - 6) - 3
      return `${x},${y}`
    })
    .join(' ')
}

function formatChartValue(value: number, unit: ChartUnit) {
  if (unit === 'currency') {
    return `$${value.toFixed(2)}`
  }

  return `${Math.round(value)}`
}

async function loadHealth() {
  loading.value = true

  try {
    const response = await fetchHealth()
    health.value = response.data
  } catch (error) {
    ElMessage.error('后端健康检查请求失败')
    console.error(error)
  } finally {
    loading.value = false
  }
}

async function handleQuery() {
  await loadHealth()
  ElMessage.success('看板数据已刷新')
}

onMounted(() => {
  void loadHealth()
})
</script>

<template>
  <section class="dashboard-view">
    <div class="dashboard-view__welcome">
      <div>
        <h1>{{ greeting }}，lhui-dev</h1>
      </div>
    </div>

    <div class="dashboard-view__filters">
      <label class="dashboard-view__field">
        <span>起始时间</span>
        <el-date-picker
          v-model="startTime"
          type="datetime"
          placeholder="选择起始时间"
        />
      </label>

      <label class="dashboard-view__field">
        <span>结束时间</span>
        <el-date-picker
          v-model="endTime"
          type="datetime"
          placeholder="选择结束时间"
        />
      </label>

      <label class="dashboard-view__field dashboard-view__field--short">
        <span>时间粒度</span>
        <el-select v-model="granularity">
          <el-option
            label="小时"
            value="hour"
          />
          <el-option
            label="天"
            value="day"
          />
          <el-option
            label="周"
            value="week"
          />
        </el-select>
      </label>

      <button
        type="button"
        class="dashboard-view__query"
        @click="handleQuery"
      >
        <el-icon><Search /></el-icon>
        <span>查询</span>
      </button>
    </div>

    <div class="dashboard-view__cards">
      <article
        v-for="card in dashboardCards"
        :key="card.title"
        class="dashboard-view__card"
      >
        <header class="dashboard-view__card-header">
          <div class="dashboard-view__card-title">
            <el-icon><component :is="card.icon" /></el-icon>
            <span>{{ card.title }}</span>
          </div>
        </header>

        <div
          class="dashboard-view__card-body"
          :class="{ 'has-chart': card.sparkline }"
        >
          <div class="dashboard-view__metric-list">
            <div
              v-for="metric in card.metrics"
              :key="metric.label"
              class="dashboard-view__metric"
            >
              <span
                class="dashboard-view__metric-icon"
                :style="{ backgroundColor: metric.color }"
              >
                <el-icon><component :is="metric.icon" /></el-icon>
              </span>
              <div class="dashboard-view__metric-copy">
                <span>{{ metric.label }}</span>
                <strong>{{ metric.value }}</strong>
              </div>
              <button
                v-if="metric.action"
                type="button"
                class="dashboard-view__metric-action"
              >
                {{ metric.action }}
              </button>
            </div>
          </div>

          <div
            v-if="card.sparkline"
            class="dashboard-view__sparklines"
          >
            <svg
              v-for="line in [card.sparkline.first, card.sparkline.second]"
              :key="line.color"
              viewBox="0 0 130 46"
              class="dashboard-view__sparkline"
            >
              <polyline
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2.5"
                :stroke="line.color"
                :points="buildSparkline(line.values)"
              />
            </svg>
          </div>
        </div>
      </article>
    </div>

    <div class="dashboard-view__bottom">
      <section class="dashboard-view__analytics">
        <header class="dashboard-view__analytics-header">
          <div class="dashboard-view__analytics-title">
            <el-icon><DataAnalysis /></el-icon>
            <span>RBAC 数据分析</span>
          </div>

          <div class="dashboard-view__analytics-tabs">
            <button
              v-for="tab in analyticsTabs"
              :key="tab.key"
              type="button"
              class="dashboard-view__analytics-tab"
              :class="{ 'is-active': tab.key === activeTab }"
              @click="activeTab = tab.key"
            >
              {{ tab.label }}
            </button>
          </div>
        </header>

        <div class="dashboard-view__chart">
          <div class="dashboard-view__chart-summary">
            <h2>{{ activeChart.title }}</h2>
            <p>总计：{{ activeChart.total }}</p>
          </div>

          <div
            v-if="activeChart.kind === 'bar'"
            class="dashboard-view__chart-grid"
          >
            <div class="dashboard-view__chart-axis">
              <span
                v-for="axisLabel in chartAxisLabels"
                :key="`${activeTab}-${axisLabel}`"
              >
                {{ axisLabel }}
              </span>
            </div>

            <div class="dashboard-view__chart-bars">
              <div class="dashboard-view__chart-guides">
                <span />
                <span />
                <span />
                <span />
              </div>

              <div class="dashboard-view__chart-columns">
                <div
                  v-for="(value, index) in activeChart.values"
                  :key="`${activeTab}-${activeChart.labels[index]}`"
                  class="dashboard-view__chart-column"
                >
                  <div class="dashboard-view__chart-bar-wrap">
                    <div
                      class="dashboard-view__chart-bar"
                      :style="{ height: `${(value / maxChartValue) * chartPlotHeight}px` }"
                    />
                  </div>
                  <span>{{ activeChart.labels[index] }}</span>
                </div>
              </div>
            </div>
          </div>

          <div
            v-else
            class="dashboard-view__ranking-list"
          >
            <div
              v-for="(value, index) in activeChart.values"
              :key="`${activeTab}-${activeChart.labels[index]}`"
              class="dashboard-view__ranking-item"
            >
              <span class="dashboard-view__ranking-order">
                {{ String(index + 1).padStart(2, '0') }}
              </span>
              <span class="dashboard-view__ranking-label">{{ activeChart.labels[index] }}</span>
              <div class="dashboard-view__ranking-track">
                <div
                  class="dashboard-view__ranking-fill"
                  :style="{ width: `${(value / maxChartValue) * 100}%` }"
                />
              </div>
              <strong class="dashboard-view__ranking-value">{{ value }} 次</strong>
            </div>
          </div>
        </div>
      </section>

      <aside class="dashboard-view__api-panel">
        <header class="dashboard-view__api-header">
          <el-icon><Connection /></el-icon>
          <span>项目进度概览</span>
        </header>

        <div class="dashboard-view__progress">
          <div class="dashboard-view__progress-summary">
            <strong>{{ progressSummary.phase }}</strong>
            <span>已完成 {{ completedProgressCount }}/{{ projectProgressItems.length }} 个主要阶段</span>
          </div>

          <div class="dashboard-view__progress-meta">
            <span>后端状态：{{ progressSummary.backendStatus }}</span>
            <span>数据库：{{ progressSummary.databaseStatus }}</span>
            <span>服务标识：{{ progressSummary.service }}</span>
          </div>

          <div class="dashboard-view__progress-list">
            <div
              v-for="item in projectProgressItems"
              :key="item.label"
              class="dashboard-view__progress-item"
            >
              <div class="dashboard-view__progress-copy">
                <strong>{{ item.label }}</strong>
                <span>{{ item.status }}</span>
              </div>
              <span class="dashboard-view__progress-percent">{{ item.progress }}%</span>
              <div class="dashboard-view__progress-track">
                <div
                  class="dashboard-view__progress-fill"
                  :style="{ width: `${item.progress}%` }"
                />
              </div>
            </div>
          </div>
        </div>
      </aside>
    </div>
  </section>
</template>

<style scoped>
.dashboard-view {
  display: grid;
  gap: 14px;
}

.dashboard-view__welcome {
  padding-top: 0;
}

.dashboard-view__welcome h1 {
  margin: 0;
  color: #0f172a;
  font-size: clamp(24px, 2.8vw, 32px);
  font-weight: 800;
  line-height: 1.25;
}

.dashboard-view__welcome p {
  margin: 6px 0 0;
  color: var(--app-text-soft);
  font-size: 12px;
}

.dashboard-view__filters {
  display: inline-grid;
  gap: 8px;
  grid-template-columns: 208px 208px 112px 88px;
  align-items: end;
  justify-content: start;
  max-width: 100%;
}

.dashboard-view__field {
  display: grid;
  gap: 4px;
}

.dashboard-view__field span {
  color: #1f2937;
  font-size: 11px;
}

.dashboard-view__field--short {
  max-width: 112px;
}

.dashboard-view__query {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  height: 34px;
  padding: 0 12px;
  border: 0;
  border-radius: 12px;
  background: #edf2f8;
  color: #334155;
  font-size: 12px;
  font-weight: 700;
  cursor: pointer;
  white-space: nowrap;
}

.dashboard-view__cards {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.dashboard-view__card,
.dashboard-view__analytics,
.dashboard-view__api-panel {
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
  overflow: hidden;
}

.dashboard-view__card {
  min-height: 188px;
}

.dashboard-view__card-header,
.dashboard-view__analytics-header,
.dashboard-view__api-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 10px 14px;
  border-bottom: 1px solid var(--app-border);
}

.dashboard-view__card-title,
.dashboard-view__analytics-title,
.dashboard-view__api-header {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  color: #111827;
  font-size: 13px;
  font-weight: 700;
}

.dashboard-view__card-body {
  padding: 12px 14px;
}

.dashboard-view__card-body.has-chart {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 118px;
  gap: 12px;
  align-items: center;
}

.dashboard-view__metric-list {
  display: grid;
  gap: 12px;
}

.dashboard-view__metric {
  display: flex;
  align-items: center;
  gap: 10px;
}

.dashboard-view__metric-icon {
  display: inline-flex;
  width: 34px;
  height: 34px;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  color: #fff;
  font-size: 18px;
}

.dashboard-view__metric-copy {
  display: grid;
  gap: 4px;
}

.dashboard-view__metric-copy span {
  color: var(--app-text-soft);
  font-size: 12px;
}

.dashboard-view__metric-copy strong {
  color: #243449;
  font-size: 17px;
  line-height: 1;
}

.dashboard-view__metric-action {
  height: 28px;
  margin-left: auto;
  padding: 0 10px;
  border: 1px solid #d8e0ea;
  border-radius: 999px;
  background: #fff;
  color: #111827;
  cursor: pointer;
}

.dashboard-view__sparklines {
  display: grid;
  gap: 12px;
}

.dashboard-view__sparkline {
  width: 100%;
  height: 40px;
}

.dashboard-view__bottom {
  display: grid;
  gap: 16px;
  grid-template-columns: minmax(0, 1.95fr) minmax(300px, 0.62fr);
}

.dashboard-view__analytics-tabs {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.dashboard-view__analytics-tab {
  padding: 0;
  border: 0;
  background: transparent;
  color: #6b7280;
  font-size: 13px;
  cursor: pointer;
}

.dashboard-view__analytics-tab.is-active {
  color: #111827;
  font-weight: 700;
}

.dashboard-view__analytics-tab + .dashboard-view__analytics-tab::before {
  content: '/';
  margin-right: 8px;
  color: #c0c8d3;
}

.dashboard-view__chart {
  padding: 16px 16px 14px;
}

.dashboard-view__chart-summary h2 {
  margin: 0;
  color: #111827;
  font-size: 15px;
}

.dashboard-view__chart-summary p {
  margin: 4px 0 0;
  color: #6b7280;
  font-size: 12px;
}

.dashboard-view__chart-grid {
  display: grid;
  grid-template-columns: 40px minmax(0, 1fr);
  gap: 8px;
  margin-top: 14px;
}

.dashboard-view__chart-axis {
  display: grid;
  justify-items: end;
  height: 222px;
  padding-bottom: 38px;
  box-sizing: border-box;
  align-content: space-between;
  color: #64748b;
  font-size: 11px;
}

.dashboard-view__chart-bars {
  position: relative;
  height: 222px;
}

.dashboard-view__chart-guides {
  position: absolute;
  inset: 0 0 38px;
  display: grid;
  align-content: space-between;
}

.dashboard-view__chart-guides span {
  display: block;
  border-top: 1px solid #edf2f7;
}

.dashboard-view__chart-columns {
  position: relative;
  z-index: 1;
  display: grid;
  grid-template-columns: repeat(6, minmax(0, 1fr));
  gap: 12px;
  height: 222px;
}

.dashboard-view__chart-column {
  display: grid;
  grid-template-rows: 184px 28px;
  gap: 10px;
  justify-items: center;
}

.dashboard-view__chart-bar-wrap {
  display: flex;
  align-items: flex-end;
  justify-content: center;
  width: 100%;
  height: 184px;
}

.dashboard-view__chart-bar {
  width: 100%;
  max-width: 70px;
  border-radius: 10px 10px 0 0;
  background: linear-gradient(180deg, #ffc21a 0%, #ffb703 100%);
}

.dashboard-view__chart-column span {
  color: #64748b;
  font-size: 11px;
  text-align: center;
  line-height: 1.2;
  word-break: break-word;
}

.dashboard-view__ranking-list {
  display: grid;
  gap: 12px;
  margin-top: 14px;
}

.dashboard-view__ranking-item {
  display: grid;
  grid-template-columns: 32px minmax(88px, 120px) minmax(0, 1fr) auto;
  align-items: center;
  gap: 12px;
}

.dashboard-view__ranking-order {
  color: #94a3b8;
  font-size: 12px;
  font-weight: 700;
}

.dashboard-view__ranking-label {
  color: #111827;
  font-size: 13px;
  font-weight: 600;
}

.dashboard-view__ranking-track {
  height: 10px;
  border-radius: 999px;
  background: #eef3f8;
  overflow: hidden;
}

.dashboard-view__ranking-fill {
  height: 100%;
  border-radius: inherit;
  background: linear-gradient(90deg, #ffc21a 0%, #ffb703 100%);
}

.dashboard-view__ranking-value {
  color: #334155;
  font-size: 12px;
  font-weight: 700;
  white-space: nowrap;
}

.dashboard-view__progress {
  display: grid;
  gap: 16px;
  padding: 18px;
}

.dashboard-view__progress-summary {
  display: grid;
  gap: 4px;
}

.dashboard-view__progress-summary strong {
  color: #111827;
  font-size: 16px;
}

.dashboard-view__progress-summary span,
.dashboard-view__progress-meta span,
.dashboard-view__progress-copy span {
  color: #64748b;
  font-size: 12px;
}

.dashboard-view__progress-meta {
  display: grid;
  gap: 8px;
  padding: 12px 14px;
  border-radius: 14px;
  background: #f8fafc;
}

.dashboard-view__progress-list {
  display: grid;
  gap: 14px;
}

.dashboard-view__progress-item {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 8px 12px;
  align-items: center;
}

.dashboard-view__progress-copy {
  display: grid;
  gap: 4px;
}

.dashboard-view__progress-copy strong {
  color: #111827;
  font-size: 13px;
}

.dashboard-view__progress-percent {
  color: #1f7aff;
  font-size: 12px;
  font-weight: 700;
}

.dashboard-view__progress-track {
  grid-column: 1 / -1;
  height: 8px;
  border-radius: 999px;
  background: #edf2f7;
  overflow: hidden;
}

.dashboard-view__progress-fill {
  height: 100%;
  border-radius: inherit;
  background: linear-gradient(90deg, #79a9ff 0%, #1f7aff 100%);
}

:deep(.dashboard-view__field .el-input__wrapper),
:deep(.dashboard-view__field .el-select__wrapper) {
  min-height: 34px;
  border-radius: 12px;
  box-shadow: none;
  background: #f6f7fb;
}

:deep(.dashboard-view__field .el-date-editor),
:deep(.dashboard-view__field .el-select) {
  width: 100%;
}

@media (max-width: 1280px) {
  .dashboard-view__cards,
  .dashboard-view__bottom {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 920px) {
  .dashboard-view__filters {
    grid-template-columns: 1fr;
  }

  .dashboard-view__field--short {
    max-width: none;
  }
}

@media (max-width: 720px) {
  .dashboard-view__card-body.has-chart {
    grid-template-columns: 1fr;
  }

  .dashboard-view__analytics-header {
    align-items: flex-start;
    flex-direction: column;
  }

  .dashboard-view__chart-grid {
    grid-template-columns: 1fr;
  }

  .dashboard-view__chart-axis {
    display: none;
  }

  .dashboard-view__chart-columns {
    gap: 10px;
  }

  .dashboard-view__ranking-item {
    grid-template-columns: 28px minmax(72px, 96px) minmax(0, 1fr) auto;
    gap: 8px;
  }
}
</style>
