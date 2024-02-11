<script setup lang="ts">
export interface ModalProps {
  closeBtn?: boolean
}
const { closeBtn } = withDefaults(defineProps<ModalProps>(), {
  closeBtn: true,
});

defineEmits<{
  close: [event: MouseEvent]
}>();

defineSlots<{
  default(props: {}): any
  header(props: {}): any
  footer(props: {}): any
}>();
</script>

<template>
  <dialog open>
    <article>
      <header v-if="typeof $slots.header === 'function'">
        <a v-if="closeBtn" aria-label="Close" class="close" @click="$emit('close', $event)"></a>
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
.close {
  cursor: pointer;
}
</style>
