<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, reactive } from 'vue';
import FarmList from './components/FarmList.vue';

const RED = '🔴';
const YELLOW = '🟡';
const GREEN = '🟢';

const colorMapping = {
  Red: RED,
  Yellow: YELLOW,
  Green: GREEN,
};

interface FarmObject {
  id: number,
  name: string,
  status: string,
  timestamp: number,
}
const farms: FarmObject[] = reactive([]);

const defaultFarms: FarmObject[] = [
  {
    "id": 0,
    "name": "Joe's Farm",
    "status": "Red",
    "timestamp": 20
  },
  {
    "id": 1,
    "name": "Sally's Farm",
    "status": "Yellow",
    "timestamp": 5
  },
];

onMounted(() => {
  invoke('load_sync').then((loadedFarms: any): void => {
    const result = loadedFarms.length > 0 ? loadedFarms : defaultFarms;
    return result.map((f: FarmObject) => {
      const status = f.status in colorMapping ? colorMapping[f.status] : RED;
      return { ...f, status };
    }).forEach((f: FarmObject) => {
      console.log(f);
      farms.push(f);
    });
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