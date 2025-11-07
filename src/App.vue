<script setup lang="ts">

import TodoList from "./components/TodoList.vue";
import TodoHeader from "./components/TodoHeader.vue";
import TodoFooter from "./components/TodoFooter.vue";

</script>

<template>
  <main class="w-full flex flex-col min-h-screen">
    <todo-header/>
    <todo-list />
    <todo-footer/>
  </main>
</template>
