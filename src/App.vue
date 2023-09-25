<script setup lang="ts">
import FarmList from './components/FarmList.vue';

</script>

<template>
  <main class="container">
    <header>
      <h1>Richland Gro-Op: Spring Crop Plan</h1>
    </header>
    <farm-list/>
  </main>
</template>

<style scoped>
main header {
  padding-top: 10vh;
  text-align: center;
}
</style>