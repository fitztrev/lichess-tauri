<script setup lang="ts">
import SystemInfo from './SystemInfo.vue'
import { useSettingsStore } from '../stores/settings'
import PageTitle from './PageTitle.vue'

const settings = useSettingsStore()
</script>

<template>
  <PageTitle>Settings</PageTitle>

  <div class="page-content">
    logged in as: bobby logout -------------- clear all data (log out, delete
    all engines, etc) --------------
    <input
      type="text"
      class="input input-bordered"
      v-model="settings.lichessHost"
    />

    <input
      type="text"
      class="input input-bordered"
      v-model="settings.externalEngineHost"
    />

    <!-- <h1 class="text-2xl my-8">Debug Info</h1>
    <SystemInfo /> -->
  </div>
</template>
