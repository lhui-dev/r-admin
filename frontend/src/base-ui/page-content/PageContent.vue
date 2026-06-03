<script setup lang="ts">
defineProps<{
  title: string
  description?: string
  eyebrow?: string
}>()
</script>

<template>
  <section class="page-content">
    <header class="page-content__header">
      <div class="page-content__copy">
        <span
          v-if="eyebrow"
          class="page-content__eyebrow"
        >
          {{ eyebrow }}
        </span>
        <h1>{{ title }}</h1>
        <p v-if="description">
          {{ description }}
        </p>
      </div>

      <div
        v-if="$slots.actions"
        class="page-content__actions"
      >
        <slot name="actions" />
      </div>
    </header>

    <div class="page-content__body">
      <slot />
    </div>
  </section>
</template>

<style scoped>
.page-content {
  display: grid;
  gap: 14px;
}

.page-content__header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
}

.page-content__copy {
  display: grid;
  gap: 6px;
}

.page-content__eyebrow {
  color: var(--app-primary);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.08em;
}

.page-content__copy h1 {
  margin: 0;
  color: #111827;
  font-size: clamp(24px, 2.8vw, 32px);
  line-height: 1.2;
}

.page-content__copy p {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 14px;
  line-height: 1.7;
}

.page-content__actions {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.page-content__body {
  display: grid;
  gap: 14px;
}

@media (max-width: 720px) {
  .page-content__header {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
