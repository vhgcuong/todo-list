<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import TodoItem from "./TodoItem.vue";
import {Todo} from "../types/models.ts";
import {fetchAll} from "../api/todo.ts";

const todos = ref<Todo[]>([]);

async function fetchTodos() {
  // todos.value = await invoke("todos");
  todos.value = await fetchAll();
}

onMounted(() => {
  fetchTodos();
})

</script>

<template>
  <div class="rounded-sm pt-2 flex-1">
    <todo-item v-for="(todo, index) in todos" :key="todo.id" :todo="todos[index]"/>
  </div>
</template>

<style scoped>

</style>