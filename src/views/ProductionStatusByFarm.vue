<script setup lang="ts">
import { ref, Teleport } from 'vue';
import useFarms, { examples } from '../farms';
import { colorList } from '../status';
import Modal from '../components/Modal.vue';
import RadioInput from '../components/RadioInput.vue';

const {
  farms, save, setFarmStatus,
} = useFarms();

const selectedFarmIndex = ref(-1);
function editFarm(i) {
  selectedFarmIndex.value = i;
}
</script>

<template>
  <teleport to="body">
    <modal v-if="selectedFarmIndex >= 0" @close="editFarm(-1)">
      <template #header>
        Edit {{ farms[selectedFarmIndex]?.name }} Status
      </template>
      <fieldset>
        <radio-input
          v-for="(c, i) in colorList"
          :label="c.title"
          :value="i"
          :checked="c.symbol === farms[selectedFarmIndex]?.status?.symbol"
          @input="setFarmStatus(selectedFarmIndex, c.symbol)"
          :key="`color-radio-${i}`"/>
      </fieldset>
    </modal>
  </teleport>
  <section>
    <fieldset class="save-btn-group">
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
    </fieldset>
    <table role="grid">
      <thead>
        <tr>
          <th colspan="3">Production Status by Farm</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(farm, i) in farms" :key="`farm-${i}`"
          @click="editFarm(i)">
          <td class="status">
            <span>
              {{ farm.status.emoji }}
            </span>
          </td>
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
fieldset.save-btn-group {
  text-align: center;
}
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
