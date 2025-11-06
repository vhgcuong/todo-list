<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
// import {Todo} from "../types/models.ts";

const todos = ref([]);

async function fetchTodos() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  todos.value = await invoke("todos");
  console.log("Todos: ", todos);
}

</script>

<template>
  <div class="rounded-sm m-8 bg-sky-500/100" @click="fetchTodos">
    m-8
  </div>
</template>

<style scoped>

</style>