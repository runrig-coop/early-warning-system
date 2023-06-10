<script setup lang="ts">
import { reactive } from 'vue';
import FarmList from './components/FarmList.vue';

const RED = '🔴';
const YELLOW = '🟡';
const GREEN = '🟢';

const oneDay = 1000 * 60 * 60 * 24;
const twoDays = oneDay * 2;
const threeDays = oneDay * 3;

const farms = reactive([
  {
    name: 'Joe\'s Farm',
    status: RED,
    timestamp: Date.now() - threeDays,
  },
  {
    name: 'Sally\'s Farm',
    status: YELLOW,
    timestamp: Date.now() - twoDays,
  },
  {
    name: 'Sam\'s Farm',
    status: GREEN,
    timestamp: Date.now() - oneDay,
  },
]);
</script>

<template>
  <div class="container">
    <h1>Richland Gro-Op Early Warning System</h1>
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
  width: 50%;
}
</style>