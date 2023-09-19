<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, reactive } from 'vue';
import { toBackendFarmObject, BackendFarmObject, FarmListItem } from './farms';
import { RED, YELLOW, toStatusObject } from './status';
import FarmList from './components/FarmList.vue';

const farms: FarmListItem[] = reactive([]);
const defaultFarms: FarmListItem[] = [
  {
    id: 0,
    name: 'Joe\'s Farm',
    status: toStatusObject(RED),
    timestamp: 20
  },
  {
    id: 1,
    name: 'Sally\'s Farm',
    status: toStatusObject(YELLOW),
    timestamp: 5
  },
];

function save() {
  const cache: BackendFarmObject[] = farms.map(toBackendFarmObject);
  invoke('save', { farms: cache });
}

onMounted(() => {
  invoke('load').then((cache: BackendFarmObject[]): void => {
    const list: FarmListItem[] = cache.map((f) => ({
      ...f, status: toStatusObject(f.status),
    }));
    if (list.length > 0) farms.push(...list);
    else farms.push(...defaultFarms);
  });
})

</script>

<template>
  <header>
    <hgroup>
      <h1>Richland Gro-Op: Spring Crop Plan</h1>
      <span role="button" @click="save" class="outline">Save</span>
    </hgroup>
  </header>
  <main class="container">
    <div class="list-container">
      <farm-list :farms="farms"></farm-list>
    </div>
  </main>
</template>

<style scoped>
header {
  padding-top: 10vh;
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