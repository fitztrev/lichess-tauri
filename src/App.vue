<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import LocalEngine from "./components/LocalEngine.vue";
</script>

<template>
  <div class="container">
    <div class="row">
        <img src="./assets/lichess-white.svg" class="logo" alt="LIchess logo" />
    </div>

    <h1>Lichess External Engine</h1>

    <div class="checklist">
      <p>&check; Connected to Lichess account: <strong>Bobby</strong> <span>&#x2699;</span></p>
      <p>&check; Engine running locally: <strong>Stockfish 15</strong> <span>&#x2699;</span></p>
      <p>&check; Engine registered on Lichess: <strong>Stockfish 15 local</strong></p>
      <p>&check; Listening for analysis requests...</p>
    </div>
  </div>

  system info:

  cpu:
  cores:
  memory:

  <LocalEngine />
</template>

<style scoped>
.checklist {
  background: #444;
  color: #59c601;
}

.checklist strong {
  border-bottom: 1px dashed;
}
.checklist span {
  color: #fff;
}
</style>
