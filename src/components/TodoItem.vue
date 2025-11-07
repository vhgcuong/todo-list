<script setup lang="ts">
import {Todo} from "../types/models.ts";
import {ref, watch} from "vue";

const {todo} = defineProps<{
  todo: Todo
}>();

const id = 'id_' + todo.id;
const name = 'check_' + todo.id;

const active = ref(todo.is_done);

watch(active, (newX) => {
  todo.is_done = newX;
})
</script>

<template>
  <div class="transition hover:border-sky-500 hover:shadow-xl shadow-md mb-2 mx-8 flex flex-col items-start divide-y divide-gray-200 rounded-xl border-2 border-sky-500/50">
    <label :for="id" class="inline-flex items-center gap-3 ml-2 mr-1 py-3">
      <input :id="id" :name="name" :checked="todo.is_done" type="checkbox"
             class="my-0.5 size-5 rounded border-gray-300 shadow-sm" @click="active = !active">
      <div>
        <p class="mt-0.5 text-sm text-gray-700" :class="{ 'line-through text-gray-700/25': active }">
          {{ todo.text }}
        </p>
      </div>
    </label>
  </div>
</template>
