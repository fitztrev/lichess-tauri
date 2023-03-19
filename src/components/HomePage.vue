<script setup lang="ts">
import { useEventLogsStore } from '../stores/event-logs'
import PageTitle from './PageTitle.vue'
import GettingStarted from './GettingStarted.vue'
import { useSettingsStore } from '../stores/settings'
import { useEnginesStore } from '../stores/engines';

const engines = useEnginesStore()
const settings = useSettingsStore()
const eventLogs = useEventLogsStore()
</script>

<template>
  <GettingStarted v-if="!settings.isLoggedIn || !engines.hasEngines" />

  <template v-else>
    <PageTitle>Dashboard</PageTitle>

    <div class="page-content">
      <div
        class="bg-white p-4 h-1/2 flex flex-col-reverse justify-end overflow-scroll font-mono"
      >
        <div
          class="whitespace-nowrap"
          v-if="eventLogs.logs.length"
          v-for="log in eventLogs.logs"
        >
          {{ log.event }} {{ log.payload.event }}:
          {{ log.payload.message }}
          {{ log.payload.analysis_request }}
        </div>

        <div v-else>No analysis requests yet</div>
      </div>
    </div>
  </template>
</template>
