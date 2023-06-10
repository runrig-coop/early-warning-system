<script setup lang="ts">
defineProps<{ farms: Array<{ name: string, status: string, timestamp: number }> }>()
</script>

<template>
  <ul class="farm-list">
    <li v-for="(farm, i) in farms" :key="`farm-${i}`" class="farm-total">
      <div class="primary-info">
        <span class="status">
          {{ farm.status }}
        </span>
        <span class="name">
          {{ farm.name }}
        </span>
      </div>
      <div class="secondary-info">
        Last Checkin: {{ new Date(farm.timestamp).toLocaleDateString() }}
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
