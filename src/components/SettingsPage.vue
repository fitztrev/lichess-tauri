<script setup lang="ts">
import SystemInfo from './SystemInfo.vue'
import { useSettingsStore } from '../stores/settings'

const settings = useSettingsStore()
</script>

<template>
  <div class="w-3/4 mx-auto my-20">
    <h1 class="text-2xl mb-8">Settings</h1>

    <div class="form-control mb-4">
      <label class="label">
        <span class="label-text">Lichess Host</span>
      </label>
      <input
        type="text"
        class="input input-bordered"
        v-model="settings.lichessHost"
      />
    </div>

    <div class="form-control mb-4">
      <label class="label">
        <span class="label-text">External Engine Host</span>
      </label>
      <input
        type="text"
        class="input input-bordered"
        v-model="settings.externalEngineHost"
      />
    </div>

    <h1 class="text-2xl my-8">Debug Info</h1>
    <SystemInfo />
  </div>
</template>
