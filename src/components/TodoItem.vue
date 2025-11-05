<script setup lang="ts">
const model = defineModel<{ text: string; isDone: boolean }>({
  default: { text: '', isDone: true }
});
</script>

<template>
  <label>
    <div>
      <input type="checkbox" class="appearance-none forced-colors:appearance-auto" :checked="model.isDone"/>
      <svg class="invisible peer-checked:visible forced-colors:hidden">
      </svg>
    </div>
    {{ model.text }}
  </label>
</template>

<style scoped>

</style>