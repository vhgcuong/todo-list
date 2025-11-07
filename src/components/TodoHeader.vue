<script setup lang="ts">
import {getDateByFormat} from "../utils/date.ts";

const currentDate = new Date();
const formatDate = getDateByFormat(currentDate, 'Y-m-d H:i:s')

</script>

<template>
  <div class="px-8 pt-5">
    <p class="text-2xl ">Todo</p>
    <p>{{ formatDate }}</p>
  </div>
</template>

<style scoped>

</style>