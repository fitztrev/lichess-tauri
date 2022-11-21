<script setup lang="ts">
import { useEventLogsStore } from '../stores/event-logs'
const eventLogs = useEventLogsStore()
</script>

<template>
  <div class="w-3/4 mx-auto my-20">
    <h1 class="text-2xl mb-8">Activity Log</h1>
    <div
      class="bg-base-200 mt-4 p-4 h-96 flex flex-col-reverse justify-end overflow-scroll"
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
