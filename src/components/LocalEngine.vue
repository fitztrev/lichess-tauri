<script setup lang="ts">
import { useEnginesStore } from '../stores/engines'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/shell'
import { useSettingsStore } from '../stores/settings'
import { useUserStore } from '../stores/user'
import { LichessWorkEvent, useEventLogsStore } from '../stores/event-logs'
import { listen } from '@tauri-apps/api/event'

const engines = useEnginesStore()
const settings = useSettingsStore()
const user = useUserStore()
const eventLogs = useEventLogsStore()

listen('lichess::work', (data: LichessWorkEvent) => {
  eventLogs.add(data)
})

function openLichess(url: string) {
  open(`${settings.lichessHost}/${url}`)
}

async function checkForAnalysisRequests() {
  let params = {
    engineHost: settings.externalEngineHost,
    apiToken: user.token,
    providerSecret: settings.providerSecret,
    engineBinaries: engines.engines.map((engine) => {
      return {
        id: engine.id,
        binary_location: engine.binaryLocation,
      }
    }),
  }
  console.log('invoking `run_engine` from app with params', { params })
  invoke('run_engine', params)
}

checkForAnalysisRequests()
</script>

<template>
  <div class="bg-emerald-700 text-emerald-100 text-center py-2">
    <svg
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
    Listening for requests from the
    <a href="#" @click.prevent="openLichess('/analysis')" class="underline"
      >Analysis page</a
    >...
  </div>
</template>
