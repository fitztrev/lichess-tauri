<script setup lang="ts">
import { open as openShell } from '@tauri-apps/api/shell'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { saveEngineToLichess } from '../utils/engine-crud'
import { RouterLink } from 'vue-router'
import { router } from '../router'
import {
  generateMaxHashOptions,
  sysinfo,
  getDefaultMaxThreadsValue,
} from '../utils/sysyinfo'
import PageTitle from './PageTitle.vue'
import Engine from './Engine.vue'
import {
  LichessEngine,
  refreshEngineList,
  useEnginesStore,
} from '../stores/engines'
import { useSettingsStore } from '../stores/settings'

const engines = useEnginesStore()
const settings = useSettingsStore()
const engineDirectory = ref<EngineListing[]>([])

function openContainingFolder(filepath: string) {
  let dir = filepath.substring(0, filepath.lastIndexOf('/'))
  openShell(dir)
}

refreshEngineList()

async function addEngineFromDirectory(engine: EngineListing) {
  let path_to_binary = await invoke<string>('download_engine_to_folder', {
    engine: engine,
  })

  sysinfo().then((systemInfo) => {
    let maxThreads = getDefaultMaxThreadsValue(systemInfo.cpus_len)

    let maxHashOptions = generateMaxHashOptions(
      systemInfo.total_memory / 1024 / 1024
    )

    saveEngineToLichess({
      name: engine.name + ' ' + engine.version,
      maxThreads: maxThreads,
      maxHash: maxHashOptions.at(-1)?.megabytes || 16,
      defaultDepth: 30,
      variants: ['chess'],
    }).then(async (data) => {
      await invoke('add_engine', {
        engineId: data.id,
        binaryLocation: path_to_binary,
      })
      refreshEngineList()
    })
  })
}

interface EngineListing {
  name: string
  description: string
  website: string
  icon: string
  license: string
  version: string
  updated_at: string
  binaries: {
    os: string
    architecture: string
    zip: string
    binary_filename: string
  }
}

fetch('https://fitztrev.github.io/lichess-tauri/engine-directory.json')
  .then<{
    engines: EngineListing[]
  }>((res) => res.json())
  .then((data) => {
    engineDirectory.value = data.engines
  })
</script>

<template>
  <PageTitle>Engines</PageTitle>

  <div class="page-content">
    <div class="overflow-hidden bg-white shadow sm:rounded-md">
      <ul role="list" class="divide-y divide-gray-200">
        <li v-for="engine in engines.engines">
          <Engine :engine="engine" />
        </li>
      </ul>
    </div>

    <div class="mx-auto max-w-lg mt-12">
      <h2 class="text-lg font-medium text-gray-900">Add an Engine</h2>
      <p class="mt-1 text-sm text-gray-500" v-if="engines.engines.length === 0">
        Get started by selecting an engine from the directory or adding your
        own.
      </p>
      <ul
        role="list"
        class="mt-6 divide-y divide-gray-200 border-t border-b border-gray-200"
      >
        <li v-for="engine in engineDirectory">
          <div class="group relative flex items-start space-x-3 py-4">
            <div class="flex-shrink-0">
              <span
                class="inline-flex items-center justify-center h-10 w-10 rounded-lg bg-gray-200"
              >
                <img :src="engine.icon" />
              </span>
            </div>
            <div class="min-w-0 flex-1">
              <div class="text-sm font-medium text-gray-900">
                <span class="absolute inset-0" aria-hidden="true"></span>
                {{ engine.name }} {{ engine.version }}
              </div>
              <p class="text-sm text-gray-500">
                {{ engine.description }}
              </p>
              <p class="text-sm text-gray-500">License: {{ engine.license }}</p>
              <p class="text-sm text-gray-500">
                {{ engine.website }}
              </p>
            </div>
          </div>
          <p>
            <a
              href="#"
              @click.prevent="addEngineFromDirectory(engine)"
              class="text-sm font-medium text-indigo-600 hover:text-indigo-500"
              >Click here to install</a
            >
          </p>
        </li>
      </ul>
      <div class="mt-6 flex">
        <router-link
          to="/engines/custom"
          class="text-sm font-medium text-indigo-600 hover:text-indigo-500"
        >
          Or add your own engine
          <span aria-hidden="true"> &rarr;</span>
        </router-link>
      </div>
    </div>
  </div>
</template>
