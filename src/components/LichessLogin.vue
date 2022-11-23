<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

async function loginViaOauth() {
  await invoke('start_oauth_server')
}
</script>

<template>
  <button
    @click="loginViaOauth"
    type="button"
    class="inline-flex items-center justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 font-medium text-blue-700 hover:bg-blue-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 sm:text-sm"
  >
    Click here to log in with Lichess
  </button>
</template>
