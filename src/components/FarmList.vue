<script setup lang="ts">
defineProps<{ farms: Array<{ name: string, status: string, timestamp: number }> }>()
</script>

<template>
  <ul class="farm-list">
    <li v-for="(farm, i) in farms" :key="`farm-${i}`" class="farm-total">
      <div class="toggle-drop-down">
        > 
      </div>
      <div class="farm-name">
        {{ farm.name }}
      </div>
      <div class="farm-total-status">
        {{ farm.status }}
      </div>
      <div class="last-checkin">
        {{ new Date(farm.timestamp).toLocaleDateString() }}
      </div>
    </li>
  </ul>
</template>

<style scoped>
.farm-list li {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
}

.farm-list li div {
  flex-grow: 1;
  max-width: 100%;
  text-align: start;
}

.farm-list li .toggle-drop-down,
.farm-list li .farm-total-status,
.farm-list li .last-checkin {
  max-width: fit-content;
}
</style>
