<script setup lang="ts">
import { ref, Teleport } from 'vue';
import useFarms, { examples } from '../farms';
import { colorList, DEFAULT_STATUS_SYMBOL, toStatusObject } from '../status';
import Modal from '../components/Modal.vue';
import RadioInput from '../components/RadioInput.vue';

const {
  farms, save, setFarmStatus,
} = useFarms();

const selectedFarmIndex = ref(-1);
function editFarm(i) {
  selectedFarmIndex.value = i;
}

const newFarmName = ref('');
const newFarmStatus = ref(DEFAULT_STATUS_SYMBOL);
const newFarmTimestamp = ref(0);
const showNewFarm = ref(false);
function initFarm() {
  newFarmName.value = '';
  newFarmStatus.value = DEFAULT_STATUS_SYMBOL;
  newFarmTimestamp.value = 0;
  showNewFarm.value = true;
}
function addFarm() {
  const id = farms.reduce((high, { id }) => id > high ? id + 1 : high, 0);
  const newFarm = {
    id,
    name: newFarmName.value,
    status: toStatusObject(newFarmStatus.value),
    timestamp: newFarmTimestamp.value,
  };
  farms.push(newFarm);
  showNewFarm.value = false;
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
    <modal v-if="showNewFarm === true" @close="showNewFarm = false">
      <template #header>Add Farm</template>
      <div class="add-farm-form">
        <fieldset class="name-field">
          <label for="farm-name">Farm Name</label>
          <input
            type="text"
            id="farm-name"
            name="farm-name"
            v-model="newFarmName">
        </fieldset>
        <fieldset class="status-field">
          <legend>Status</legend>
          <radio-input
            v-for="(c, i) in colorList"
            :label="c.title"
            :value="i"
            :checked="c.symbol === newFarmStatus"
            @input="newFarmStatus = c.symbol"/>
        </fieldset>
        <fieldset class="time-field">
          <label for="farm-timestamp">Days Since Last Status Update</label>
          <input
            type="number"
            id="farm-timestamp"
            name="farm-timestamp"
            v-model="newFarmTimestamp">
        </fieldset>
      </div>
      <template #footer>
        <span
          role="button"
          @click="addFarm">
          Add
        </span>
        <span
          role="button"
          @click="showNewFarm = false"
          class="secondary">
          Discard
        </span>
      </template>
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
    <span role="button" @click="initFarm">Add Farm</span>
  </section>
</template>

<style scoped>
fieldset.save-btn-group {
  text-align: center;
}

fieldset.name-field {
  grid-area: name-field;
}
fieldset.status-field {
  grid-area: status-field;
}
fieldset.time-field {
  grid-area: time-field;
}
.add-farm-form {
  display: grid;
  grid-template-columns: 50% 50%;
  grid-template-rows: auto;
  grid-template-areas: 
    "name-field name-field"
    "status-field time-field";
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
