<script setup lang="ts">
import { reactive } from 'vue';
import FarmList from './components/FarmList.vue';

const RED = '🔴';
const YELLOW = '🟡';
const GREEN = '🟢';

const farms = reactive([
  {
    name: 'Joe\'s Farm',
    status: RED,
    timestamp: 10,
  },
  {
    name: 'Sally\'s Farm',
    status: YELLOW,
    timestamp: 5,
  },
  {
    name: 'Joe\'s Other Farm',
    status: YELLOW,
    timestamp: 4,
  },
  {
    name: 'Sam\'s Farm',
    status: GREEN,
    timestamp: 0,
  },
]);
</script>

<template>
  <div class="container">
    <h1>Richland Gro-Op: Spring Crop Plan</h1>
    <div class="list-container">
      <farm-list :farms="farms"></farm-list>
    </div>
  </div>
</template>

<style scoped>
.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.list-container {
  display: flex;
  flex-direction: row;
  justify-content: center;
}
.list-container ul {
  width: 60%;
}
</style>