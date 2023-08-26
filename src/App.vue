<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import StatusBar from './components/StatusBar.vue'
import {
  LichessStatusEvent,
  LichessWorkEvent,
  useAnalysisStore,
} from './stores/analysis'
import { useSettingsStore } from './stores/settings'
import { loadSettingsFromDatabase } from './utils/settings'

const analysis = useAnalysisStore()
const settings = useSettingsStore()

listen('lichess::work', (data: LichessWorkEvent) => {
  analysis.statusLevel = 'Info'
  analysis.handle(data)
})

listen('lichess::send_status_to_frontend', (data: LichessStatusEvent) => {
  analysis.status = data.payload.status
  analysis.statusLevel = data.payload.level
})

listen('refresh_settings_from_database', () => {
  loadSettingsFromDatabase()
})
</script>

<template>
  <div>
    <div
      class="hidden lg:fixed lg:inset-y-0 lg:left-0 lg:z-50 lg:block lg:w-20 lg:overflow-y-auto lg:bg-gray-900 lg:pb-4"
    >
      <div class="flex h-16 shrink-0 items-center justify-center">
        <img
          class="h-8 w-auto"
          src="./assets/lichess-white.svg"
          alt="Lichess logo"
        />
      </div>
      <nav class="mt-8">
        <ul role="list" class="flex flex-col items-center space-y-1">
          <li>
            <!-- Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" -->
            <router-link to="/" custom v-slot="{ href, isActive, navigate }"
              ><a
                :href="href"
                @click="navigate"
                class="group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold"
                :class="{
                  'bg-gray-800 text-white': isActive,
                  'text-gray-400 hover:text-white hover:bg-gray-800': !isActive,
                }"
              >
                <svg
                  class="h-6 w-6 shrink-0"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
                  />
                </svg>
                <span class="sr-only">Dashboard</span>
              </a></router-link
            >
          </li>
          <li>
            <router-link
              v-if="settings.isLoggedIn"
              to="/engines"
              custom
              v-slot="{ href, isActive, navigate }"
              ><a
                :href="href"
                @click="navigate"
                class="group flex gap-x-3 rounded-md p-3 text-sm leading-6 font-semibold"
                :class="{
                  'bg-gray-800 text-white': isActive,
                  'text-gray-400 hover:text-white hover:bg-gray-800': !isActive,
                }"
              >
                <svg
                  class="h-6 w-6 shrink-0"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M10.5 6h9.75M10.5 6a1.5 1.5 0 11-3 0m3 0a1.5 1.5 0 10-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-9.75 0h9.75"
                  />
                </svg>
                <span class="sr-only">Dashboard</span>
              </a></router-link
            >
          </li>

          <li>
            <router-link
              to="/settings"
              custom
              v-slot="{ href, isActive, navigate }"
              ><a
                :href="href"
                @click="navigate"
                class="group flex items-center px-2 py-2 text-sm font-medium rounded-md"
                :class="{
                  'bg-gray-900 text-white': isActive,
                  'text-gray-300 hover:bg-gray-700 hover:text-white': !isActive,
                }"
              >
                <svg
                  class="h-6 w-6 shrink-0"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z"
                  />
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                  />
                </svg>
                <span class="sr-only">Dashboard</span>
              </a></router-link
            >
          </li>
        </ul>
      </nav>
    </div>

    <div class="lg:pl-20">
      <main class="">
        <div class="">
          <!-- Main area -->
          <StatusBar v-if="analysis.status" />
          <div class="py-6">
            <router-view />
          </div>
        </div>
      </main>
    </div>
  </div>
</template>
