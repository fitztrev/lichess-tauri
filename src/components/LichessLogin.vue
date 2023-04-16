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
    class="inline-flex items-center justify-center rounded-md border border-transparent px-4 py-2 font-medium bg-indigo-100 text-indigo-800 hover:bg-indigo-500 hover:text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:text-sm"
  >
    Click here to log in with Lichess
  </button>
</template>
