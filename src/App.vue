<script setup lang="ts">
import { onMounted, reactive } from 'vue';
import FarmList from './components/FarmList.vue';

const RED = '🔴';
const YELLOW = '🟡';
const GREEN = '🟢';

interface FarmObject {
  name: string,
  status: string,
  timestamp: number,
}
const farms: FarmObject[] = reactive([]);

onMounted(() => {
  farms.push({
    name: 'Joe\'s Farm',
    status: RED,
    timestamp: 10,
  });
  farms.push({
    name: 'Sally\'s Farm',
    status: YELLOW,
    timestamp: 5,
  });
  farms.push({
    name: 'Joe\'s Other Farm',
    status: YELLOW,
    timestamp: 4,
  });
  farms.push({
    name: 'Sam\'s Farm',
    status: GREEN,
    timestamp: 0,
  });
})

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