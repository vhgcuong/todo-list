<script setup lang="ts">
import {Todo} from "../types/models.ts";

const { todo } = defineProps<{
  todo: Todo
}>()
</script>

<template>
  <div class="flex items-center space-x-2 border-y-indigo-500 p-3">
    <input :checked="todo.is_done" type="checkbox" id="example1" class="h-4 w-4 rounded border-gray-300 text-primary-600 shadow-sm focus:border-primary-300 focus:ring focus:ring-primary-200 focus:ring-opacity-50 focus:ring-offset-0 disabled:cursor-not-allowed disabled:text-gray-400" />
    <label for="example1" class="text-sm font-medium text-gray-700">{{ todo.text }}</label>
  </div>
</template>
