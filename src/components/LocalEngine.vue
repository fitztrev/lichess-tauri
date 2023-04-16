<script setup lang="ts">
import { LichessWorkEvent, useAnalysisStore } from '../stores/analysis'
import { listen } from '@tauri-apps/api/event'
import SleepCountdown from './SleepCountdown.vue'

const analysis = useAnalysisStore()

listen('lichess::work', (data: LichessWorkEvent) => {
  analysis.handle(data)
})
</script>

<template>
  <div class="bg-emerald-700 text-white text-center py-2">
    <svg
      v-if="!analysis.sleepDuration"
      class="animate-spin inline-block -ml-1 mr-3 h-5 w-5 text-white"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      ></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      ></path>
    </svg>

    {{ analysis.status }}

    <SleepCountdown
      v-if="analysis.sleepDuration > 0"
      :duration="analysis.sleepDuration"
    />
  </div>
</template>
