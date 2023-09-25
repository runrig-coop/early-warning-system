<script setup lang="ts">
import { computed } from 'vue';

const {
  value, checked, label
} = defineProps<{
  value: any,
  checked: boolean,
  label: string,
}>();
console.log('labelText', label);
const id = computed(() => label.toLowerCase());

const emit = defineEmits<{
  input: [id: string, value: any]
}>()
</script>

<template>
  <span class="input-group">
    <input
      type="radio"
      :id="id"
      :name="id"
      :value="value"
      :checked="checked"
      @input="emit('input', id, value)">
    <label :for="id">{{ label }}</label>
  </span>
</template>

<style scoped>
.input-group {
  display: flex;
  flex-flow: row;
}
</style>