<script setup lang="ts">
import { FarmListItem } from '../farms';

defineProps<{ farms: FarmListItem[] }>()
</script>

<template>
  <section>
    <table role="grid">
      <thead>
        <tr>
          <th colspan="3">Production Status by Farm</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(farm, i) in farms" :key="`farm-${i}`">
          <td class="status">{{ farm.status.emoji }}</td>
          <td class="name">{{ farm.name }}</td>
          <td class="secondary-info">
            Last report: {{
              farm.timestamp === 0 ? 'Today' : `${farm.timestamp} days ago`
            }}
          </td>
        </tr>
      </tbody>
    </table>
  </section>
</template>

<style scoped>
th[colspan="3"] {
  text-align: center;
  font-size: 1.75rem;
}

.secondary-info {
  color: #777;
}

.status {
  font-size: 2rem;
}

.name {
  font-size: 1.5rem;
}
</style>
