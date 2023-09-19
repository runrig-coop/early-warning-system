<script setup lang="ts">
defineProps<{farms: Array<{
  id: number,
  name: string,
  status: {
    emoji: string,
    symbol: symbol,
    title: string,
  },
  timestamp: number,
}>}>()
</script>

<template>
  <ul class="farm-list">
    <h2>Production Status by Farm</h2>
    <li v-for="(farm, i) in farms" :key="`farm-${i}`" class="farm-total">
      <div class="primary-info">
        <span class="status">
          {{ farm.status.emoji }}
        </span>
        <span class="name">
          {{ farm.name }}
        </span>
      </div>
      <div v-if="farm.timestamp === 0" class="secondary-info">
        Last report: Today
      </div>
      <div v-else class="secondary-info">
        Last report: {{ farm.timestamp }} days ago
      </div>
    </li>
  </ul>
</template>

<style scoped>
ul.farm-list {
  display: flex;
  flex-direction: column;
}
.farm-list li {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  padding: 1.5rem;
}

.farm-list li .primary-info {
  flex-grow: 1;
  max-width: 100%;
  text-align: start;
}

.farm-list li .toggle-drop-down,
.farm-list li .farm-total-status,
.farm-list li .secondary-info {
  max-width: fit-content;
}

.secondary-info {
  color: #777;
}

.status {
  font-size: 2rem;
  padding-right: 1.5rem;
}

.name {
  font-size: 1.5rem;
  padding-right: 1.5rem;
}
</style>
