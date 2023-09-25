<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, provide, reactive, ref } from 'vue';
import { toBackendFarmObject, BackendFarmObject, FarmListItem } from './farms';
import { colorMap, GREEN, RED, YELLOW, toStatusObject } from './status';
import FarmList from './components/FarmList.vue';
import Modal from './components/Modal.vue';

const farms: FarmListItem[] = reactive([]);

function save() {
  const cache: BackendFarmObject[] = farms.map(toBackendFarmObject);
  invoke('save', { farms: cache });
}

const selectedFarmIndex = ref(-1);
function editFarm(i) {
  selectedFarmIndex.value = i;
}
provide('editFarm', editFarm);

const colors = Object.getOwnPropertySymbols(colorMap)
  .map(symbol => ({ ...colorMap[symbol], symbol }));

function setFarmStatus(sym: symbol) {
  farms[selectedFarmIndex.value].status = toStatusObject(sym);
}

onMounted(() => {
  invoke('load').then((cache: BackendFarmObject[]): void => {
    const list: FarmListItem[] = cache.map((f) => ({
      ...f, status: toStatusObject(f.status),
    }));
    farms.push(...list);
  });
})

const examples: FarmListItem[] = [
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
  {
    id: 2,
    name: 'Sam\'s Farm',
    status: toStatusObject(GREEN),
    timestamp: 0
  },
];

</script>

<template>
  <modal v-if="selectedFarmIndex >= 0" @close="editFarm(-1)">
    <template #header>
      Edit {{ farms[selectedFarmIndex]?.name }} Status
    </template>
    <fieldset>
      <span v-for="(c, i) in colors" class="input-group" :key="`color-radio-${i}`">
        <input
          type="radio"
          :id="c.title"
          :name="c.title"
          :value="i"
          :checked="c.symbol === farms[selectedFarmIndex]?.status?.symbol"
          @input="setFarmStatus(c.symbol)">
        <label :for="c.title">{{ c.title }}</label>
      </span>
    </fieldset>
  </modal>
  <main class="container">
    <header>
      <hgroup>
        <h1>Richland Gro-Op: Spring Crop Plan</h1>
        <span role="button" @click="save" class="outline">Save</span>
        &nbsp;
        <span role="button"
          v-if="farms.length === 0"
          @click="farms.push(...examples)"
          class="outline secondary">
          Examples
        </span>
        &nbsp;
        <span
          role="button"
          v-if="farms.length > 0"
          @click="farms.splice(0)"
          class="outline secondary">
          Clear
        </span>
      </hgroup>
    </header>
    <div class="list-container">
      <farm-list :farms="farms"></farm-list>
    </div>
  </main>
</template>

<style scoped>
main header {
  padding-top: 10vh;
  text-align: center;
}

.input-group {
  display: flex;
  flex-flow: row;
}
</style>