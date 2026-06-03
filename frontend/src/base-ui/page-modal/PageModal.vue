<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue: boolean
  title: string
  width?: string | number
  closeOnClickModal?: boolean
}>(), {
  width: 720,
  closeOnClickModal: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  cancel: []
}>()

function handleClose() {
  emit('update:modelValue', false)
  emit('cancel')
}
</script>

<template>
  <el-dialog
    :model-value="props.modelValue"
    :title="props.title"
    :width="props.width"
    :close-on-click-modal="props.closeOnClickModal"
    destroy-on-close
    @close="handleClose"
  >
    <slot />

    <template #footer>
      <slot name="footer" />
    </template>
  </el-dialog>
</template>
