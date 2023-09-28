<script setup lang="ts">
import { reactive, ref, Teleport } from 'vue';
import useFarms, { examples } from '../farms';
import { colorList, DEFAULT_STATUS_SYMBOL, toStatusObject } from '../status';
import Modal from '../components/Modal.vue';
import RadioInput from '../components/RadioInput.vue';
import IconDelete from '../components/IconDelete.vue';
import IconEdit from '../components/IconEdit.vue';

const {
  farms, save, setFarmName, setFarmStatus,
} = useFarms();

const selectedFarmIndex = ref(-1);
function editFarm(i) {
  selectedFarmIndex.value = i;
}

const selectedFarmName = reactive({ index: -1, name: '' });
function editFarmName(i, name) {
  selectedFarmName.name = name;
  selectedFarmName.index = i;
}
function updateFarmName() {
  const { index, name } = selectedFarmName;
  setFarmName(index, name);
  editFarmName(-1, '');
}

const readyToDelete = ref(false);
function confirmDelete(bool) {
  if (bool) farms.splice(selectedFarmName.index, 1);
  readyToDelete.value = false;
  editFarmName(-1, '');
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
        {{ farms[selectedFarmIndex]?.name }} Status
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
    <modal
      v-if="selectedFarmName.index >= 0 && !readyToDelete"
      @close="editFarmName(-1, '')">
      <template #header>Change Name</template>
      <fieldset>
        <input type="text" v-model="selectedFarmName.name">
      </fieldset>
      <template #footer>
        <span
          @click="readyToDelete = true"
          class="delete-farm">
          <icon-delete/>
        </span>
        <span
          role="button"
          @click="updateFarmName">
          Save
        </span>
        <span
          role="button"
          @click="editFarmName(-1, '')"
          class="secondary">
          Cancel
        </span>
      </template>
    </modal>
    <modal v-if="readyToDelete">
      <template #header>Change Name</template>
      <p>Permanently delete {{ selectedFarmName.name }}?</p>
      <template #footer>
        <span
          role="button"
          @click="confirmDelete(true)">
          Delete
        </span>
        <span
          role="button"
          @click="confirmDelete(false)"
          class="secondary">
          Cancel
        </span>
      </template>
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
          <td class="name">
            <span class="name-span">
              {{ farm.name }}
              <span class="edit-icon" @click.stop="editFarmName(i, farm.name)">
                <icon-edit/>
              </span>
            </span>
          </td>
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
tr:hover {
  cursor: pointer;
  box-shadow: 0px 0px 3px var(--primary-focus);
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
.name-span {
  display: inline-block;
  position: relative;
}
span.edit-icon {
  display: block;
  position: absolute;
  cursor: pointer;
  top: calc(50% - 1.5rem);
  right: calc(-2 * 1.5rem);
  height: calc(1.5rem * 2);
  padding: calc(1.5rem * .5);
}
span.edit-icon svg {
  display: block;
  height: 1.5rem;
  width: 1.5rem;
  stroke: var(--primary);
  fill: none;
}
span.edit-icon:hover svg {
  filter: drop-shadow(0px 0px 3px var(--primary-focus));
  stroke-width: 1.25px;
}

.delete-farm {
  padding: calc(1.5rem * .5);
  cursor: pointer;
}
.delete-farm svg {
  fill: var(--del-color);
  height: 1.5rem;
  width: 1.5rem;
}
</style>
