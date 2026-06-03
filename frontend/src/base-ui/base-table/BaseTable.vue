<script setup lang="ts">
const props = withDefaults(defineProps<{
  total: number
  page: number
  pageSize: number
  pageSizes?: number[]
  showPagination?: boolean
}>(), {
  pageSizes: () => [10, 20, 50, 100],
  showPagination: true,
})

const emit = defineEmits<{
  'update:page': [value: number]
  'update:pageSize': [value: number]
}>()

function handleCurrentChange(value: number) {
  emit('update:page', value)
}

function handleSizeChange(value: number) {
  emit('update:pageSize', value)
}
</script>

<template>
  <section class="base-table">
    <div
      v-if="$slots.toolbar"
      class="base-table__toolbar"
    >
      <slot name="toolbar" />
    </div>

    <div class="base-table__body">
      <slot />
    </div>

    <div
      v-if="showPagination"
      class="base-table__footer"
    >
      <el-pagination
        background
        layout="total, sizes, prev, pager, next"
        :total="props.total"
        :current-page="props.page"
        :page-size="props.pageSize"
        :page-sizes="props.pageSizes"
        @current-change="handleCurrentChange"
        @size-change="handleSizeChange"
      />
    </div>
  </section>
</template>

<style scoped>
.base-table {
  display: grid;
  gap: 14px;
  padding: 16px;
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
}

.base-table__toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.base-table__body {
  min-width: 0;
}

.base-table__footer {
  display: flex;
  justify-content: flex-end;
}
</style>
