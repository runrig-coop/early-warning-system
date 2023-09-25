<script setup lang="ts">
import ProductionStatusByFarm from './components/ProductionStatusByFarm.vue';

</script>

<template>
  <main class="container">
    <header>
      <h1>Richland Gro-Op: Spring Crop Plan</h1>
    </header>
    <production-status-by-farm/>
  </main>
</template>

<style scoped>
main header {
  padding-top: 10vh;
  text-align: center;
}
</style>