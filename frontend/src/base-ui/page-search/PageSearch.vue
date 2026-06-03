<template>
  <section class="page-search">
    <div class="page-search__fields">
      <slot />
    </div>

    <div
      v-if="$slots.actions"
      class="page-search__actions"
    >
      <slot name="actions" />
    </div>
  </section>
</template>

<style scoped>
.page-search {
  display: grid;
  gap: 14px;
  padding: 16px;
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
}

.page-search__fields {
  display: grid;
  gap: 12px;
}

.page-search__actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  flex-wrap: wrap;
}
</style>
