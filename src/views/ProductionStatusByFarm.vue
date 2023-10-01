<script setup lang="ts">
import { computed, reactive, ref, Teleport } from 'vue';
import useFarms, { examples } from '../farms';
import { colorList, DEFAULT_STATUS_SYMBOL, toStatusObject } from '../status';
import Modal from '../components/Modal.vue';
import RadioInput from '../components/RadioInput.vue';
import IconDelete from '../components/IconDelete.vue';
import IconEdit from '../components/IconEdit.vue';

const { farms, save } = useFarms();

enum ModalState {
  Closed = 'CLOSED',
  UpdateStatus = 'UPDATE_STATUS',
  UpdateName = 'UPDATE_NAME',
  CreateFarm = 'CREATE_FARM',
  DeleteFarm = 'DELETE_FARM',
}
const modalState = ref<ModalState>(ModalState.Closed);

const generateId = () => farms.reduce((hi, { id }) => id > hi ? id + 1 : hi, 0);
type SelectedFarm = { index: number; name: string; status: symbol; timestamp: number };
const farmDefaults: SelectedFarm = {
  index: -1,
  name: '',
  status: DEFAULT_STATUS_SYMBOL,
  timestamp: 0,
}
const selectedFarm = reactive<SelectedFarm>({...farmDefaults});
function selectFarm(i: number = farms.length, modal?: ModalState) {
  selectedFarm.index = i;
  selectedFarm.name = farms[i]?.name || farmDefaults.name;
  selectedFarm.status = farms[i]?.status?.symbol || farmDefaults.status;
  selectedFarm.timestamp = farms[i]?.timestamp || farmDefaults.timestamp;
  modalState.value = modal || ModalState.Closed;
}
function editFarm<K extends keyof SelectedFarm, V extends SelectedFarm[K]>(key: K, value: V) {
  selectedFarm[key] = value;
}

function upsert() {
  const { index = farms.length, name, timestamp } = selectedFarm;
  const status = toStatusObject(selectedFarm.status);
  const id = index in farms ? farms[index].id : generateId();
  farms.splice(index, index < farms.length ? 1 : 0, { id, name, status, timestamp });
  modalState.value = ModalState.Closed;
  save();
}
function confirmDelete(bool) {
  if (bool) farms.splice(selectedFarm.index, 1);
  editFarm('index', -1);
  modalState.value = ModalState.Closed;
  save();
}
function addExamples() {
  farms.push(...examples);
  save();
}
function clearAllFarms() {
  farms.splice(0);
  save();
}

const modalHeader = reactive<{ [key in ModalState]?: string }>({
  [ModalState.UpdateStatus]: `${selectedFarm.name} Status`,
  [ModalState.UpdateName]: 'Change Farm Name',
  [ModalState.CreateFarm]: 'Add Farm',
  [ModalState.DeleteFarm]: 'Confirm Deletion',
});

// These modal states render the same pair of buttons: the primary will upsert
// the selected farm then close; the secondary will cancel the action then close.
const useUpsertOrCloseFooter = computed(() => [
  ModalState.UpdateName,
  ModalState.UpdateStatus,
  ModalState.CreateFarm,
].includes(modalState.value));

const rowHoverRefs = ref<boolean[]>([]);
const setRowHoverRef = (i: number, b: boolean) => { rowHoverRefs.value[i] = b; };
</script>

<template>
  <!-- Use Vue.Teleport to append it to the document's body element. -->
  <teleport to="body">

    <!-- MODAL: Where the selected farm's properties are edited. -->
    <modal v-if="modalState !== ModalState.Closed">
      <!-- MODAL HEADER -->
      <template #header>
        {{ modalHeader[modalState] }}
      </template>

      <!-- MODAL BODY -->
      <template #default>
        <!-- UPDATE STATUS FORM -->
        <fieldset v-if="modalState === ModalState.UpdateStatus">
          <radio-input
            v-for="(c, i) in colorList"
            :label="c.title"
            :value="i"
            :checked="c.symbol === selectedFarm.status"
            @input="editFarm('status', c.symbol)"
            :key="`color-radio-${i}`"/>
        </fieldset>
  
        <!-- UPDATE NAME FORM -->
        <fieldset v-if="modalState === ModalState.UpdateName">
          <input type="text" v-model="selectedFarm.name">
        </fieldset>
  
        <!-- DELETE CONFIRMATION MESSAGE -->
        <p v-if="modalState === ModalState.DeleteFarm">
          Permanently delete {{ selectedFarm.name }}?
        </p>
  
        <!-- ADD FARM FORM -->
        <div class="add-farm-form" v-if="modalState === ModalState.CreateFarm">
          <fieldset class="name-field">
            <label for="farm-name">Farm Name</label>
            <input
              type="text"
              id="farm-name"
              name="farm-name"
              v-model="selectedFarm.name">
          </fieldset>
          <fieldset class="status-field">
            <legend>Status</legend>
            <radio-input
              v-for="(c, i) in colorList"
              :label="c.title"
              :value="i"
              :checked="c.symbol === selectedFarm.status"
              @input="selectedFarm.status = c.symbol"/>
          </fieldset>
          <fieldset class="time-field">
            <label for="farm-timestamp">Days Since Last Status Update</label>
            <input
              type="number"
              id="farm-timestamp"
              name="farm-timestamp"
              v-model="selectedFarm.timestamp">
          </fieldset>
        </div>
      </template>

      <!-- MODAL FOOTER: UpdateStatus, UpdateName & CreateFarm  -->
      <template #footer v-if="useUpsertOrCloseFooter">
        <span
          v-if="modalState === ModalState.UpdateName"
          @click="modalState = ModalState.DeleteFarm"
          class="delete-farm">
          <icon-delete/>
        </span>
        <span
          role="button"
          @click="upsert">
          {{ modalState === ModalState.CreateFarm ? 'Add' : 'Save' }}
        </span>
        <span
          role="button"
          @close="modalState = ModalState.Closed"
          class="secondary">
          {{ modalState === ModalState.CreateFarm ? 'Discard' : 'Cancel' }}
        </span>
      </template>

      <!-- MODAL FOOTER: DeleteFarm -->
      <template #footer v-if="modalState === ModalState.DeleteFarm">
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
  </teleport>

  <!-- LIST OF FARMS AND THEIR PRODUCTION STATUS -->
  <section>
    <table role="grid">
      <thead>
        <tr>
          <th colspan="3">Production Status by Farm</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(farm, i) in farms" :key="`farm-${i}`"
          @mouseenter="setRowHoverRef(i, true)"
          @mouseleave="setRowHoverRef(i, false)"
          @click="selectFarm(i, ModalState.UpdateStatus)">
          <td class="status">
            <span>
              {{ farm.status.emoji }}
            </span>
          </td>
          <td class="name">
            <span class="name-span">
              {{ farm.name }}
              <span
                v-if="rowHoverRefs[i]"
                class="edit-icon"
                @click.stop="selectFarm(i, ModalState.UpdateName)">
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
    <span
      role="button"
      @click="selectFarm(farms.length, ModalState.CreateFarm)">
      Add Farm
    </span>
    &nbsp;
    <span role="button"
      v-if="farms.length === 0"
      @click="addExamples"
      class="outline secondary">
      Add Examples
    </span>
    &nbsp;
    <span
      role="button"
      v-if="farms.length > 0"
      @click="clearAllFarms"
      class="outline secondary">
      Clear All Farms
    </span>
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
tr:hover:not(thead tr) {
  cursor: pointer;
}
tr:hover:not(thead tr) td:first-child {
  position: relative;
}
tr:hover:not(thead tr) td:first-child::before {
  position: absolute;
  content: "";
  top: 0;
  left: -8px;
  height: 100%;
  width: 4px;
  border-radius: 25% / 4px;
  background-color: var(--primary-focus);
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
