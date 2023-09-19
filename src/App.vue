<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, reactive } from 'vue';
import FarmList from './components/FarmList.vue';

const RED = Symbol('RED');
const YELLOW = Symbol('YELLOW');
const GREEN = Symbol('GREEN');
const DEFAULT_STATUS_SYMBOL = RED;

interface ColorMap {
  [color: symbol]: { title: string, emoji: string }
}
const colorMap: ColorMap = {
  [RED]: {
    emoji: '🔴',
    title: 'Red',
  },
  [YELLOW]: {
    emoji: '🟡',
    title: 'Yellow',
  },
  [GREEN]: {
    emoji: '🟢',
    title: 'Green',
  },
};
interface StatusObject {
  symbol: symbol,
  emoji: string,
  title: string,
}
const fromSymbol = (symbol: symbol): StatusObject => {
  if (symbol in colorMap) return {
    emoji: colorMap[symbol].emoji,
    symbol,
    title: colorMap[symbol].title,
  };
  return fromSymbol(DEFAULT_STATUS_SYMBOL);
};
const defaultStatus: StatusObject = fromSymbol(DEFAULT_STATUS_SYMBOL);
const colors: StatusObject[] = Object.getOwnPropertySymbols(colorMap).map(fromSymbol);
const fromAttr = (attr?: string): StatusObject =>
  colors.find(c => attr === c.title || attr === c.emoji) || defaultStatus;
const toStatusObject = (status?: symbol|string): StatusObject =>
  typeof status === 'symbol' ? fromSymbol(status) : fromAttr(status);

interface BackendFarmObject {
  id: number,
  name: string,
  status: string,
  timestamp: number,
}
interface FarmListItem {
  id: number,
  name: string,
  status: StatusObject,
  timestamp: number,
}
const toBackendFarmObject = (f: FarmListItem): BackendFarmObject =>
  ({ ...f, status: f.status.title });

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
  <div class="container">
    <h1>Richland Gro-Op: Spring Crop Plan</h1>
    <div>
      <button @click="save">Save</button>
    </div>
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