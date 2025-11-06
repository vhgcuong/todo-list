<script setup lang="ts">
import {Todo} from "../types/models.ts";

const { todo } = defineProps<{
  todo: Todo
}>()
</script>

<template>
    <div >
      <input type="checkbox" class=" " :checked="todo.is_done"/>

    {{ todo.text }}
    </div>
</template>
