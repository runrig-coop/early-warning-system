<script setup lang="ts">
export interface ModalProps {
  closeBtn?: boolean
}
const { closeBtn } = withDefaults(defineProps<ModalProps>(), {
  closeBtn: true,
});

defineEmits<{
  close: [event: EventTarget]
}>();

defineSlots<{
  default(): any
  header(): any
  footer(): any
}>();
</script>

<template>
  <dialog open>
    <article>
      <header v-if="typeof $slots.header === 'function'">
        <a v-if="closeBtn" aria-label="Close" class="close" @click="$emit('close')"></a>
        <slot name="header"></slot>
      </header>
      <slot></slot>
      <footer v-if="(typeof $slots.footer === 'function')">
        <slot name="footer"></slot>
      </footer>
    </article>
  </dialog>
</template>

<style scoped>

</style>
