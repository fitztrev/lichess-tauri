<script setup lang="ts">
import { open as openShell } from '@tauri-apps/api/shell'
import { open as openDialog } from '@tauri-apps/api/dialog'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { saveEngineToLichess } from '../utils/engine-crud'
import { RouterLink } from 'vue-router'
import { router } from '../router'
import { sysinfo } from '../utils/sysyinfo'
import PageTitle from './PageTitle.vue'
import { LichessEngine, useEnginesStore } from '../stores/engines'
import { useSettingsStore } from '../stores/settings'

const engines = useEnginesStore()
const settings = useSettingsStore()
const engineDirectory = ref<EngineListing[]>([])

function openContainingFolder(filepath: string) {
  let dir = filepath.substring(0, filepath.lastIndexOf('/'))
  openShell(dir)
}

async function getUserEnginesFromLichess(): Promise<LichessEngine[]> {
  let url = `${settings.lichessHost}/api/external-engine`
  return await fetch(url, {
    headers: {
      Authorization: `Bearer ${settings.token}`,
    },
  }).then<LichessEngine[]>((response) => response.json())
}

function refreshEngineList(): void {
  getUserEnginesFromLichess().then((data) => {
    engines.engines = data
  })
}

refreshEngineList()

async function addEngineFromDirectory(engine: EngineListing) {
  const folderPath = await openDialog({
    title: 'Select the folder where you want to save the engine',
    directory: true,
  })

  if (!folderPath) return // user cancelled the dialog

  let path_to_binary = await invoke<string>('download_engine_to_folder', {
    engine: engine,
    folder: folderPath,
  })

  sysinfo().then((systemInfo) => {
    let maxThreads = systemInfo.cpus_len
    let maxHash = 16

    let memoryLimit = (systemInfo.total_memory / 1024 / 1024) * 0.7 // up to 70% of total memory
    for (let i = 16; i <= memoryLimit; i *= 2) {
      maxHash = i
    }

    saveEngineToLichess({
      name: engine.name + ' ' + engine.version,
      maxThreads: maxThreads,
      maxHash: maxHash,
      defaultDepth: 25,
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
          <a href="#" class="block hover:bg-gray-50">
            <div class="px-4 py-4 sm:px-6">
              <div class="flex items-center justify-between">
                <p class="truncate text-sm font-medium text-indigo-600">
                  {{ engine.name }}
                </p>
                <div class="ml-2 flex flex-shrink-0">
                  <p
                    v-for="variant in engine.variants"
                    class="inline-flex rounded-full bg-green-100 px-2 text-xs font-semibold leading-5 text-green-800"
                  >
                    {{ variant }}
                  </p>
                </div>
              </div>
              <div class="mt-2 sm:flex sm:justify-between">
                <div class="sm:flex">
                  <p class="flex items-center text-sm text-gray-500">
                    Max:
                    {{ engine.maxHash }} MB &bullet;
                    {{ engine.maxThreads }} threads
                    <br />
                    Default:
                    {{ engine.defaultDepth }} depth
                  </p>
                </div>
                <div
                  class="mt-2 flex items-center text-sm text-gray-500 sm:mt-0"
                >
                  <p>
                    <!-- {{ engine.binaryLocation }} -->
                  </p>
                </div>
              </div>
            </div>
          </a>
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
        <li
          v-for="engine in engineDirectory"
          @click="addEngineFromDirectory(engine)"
        >
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
                <a href="#">
                  <span class="absolute inset-0" aria-hidden="true"></span>
                  {{ engine.name }} {{ engine.version }}
                </a>
              </div>
              <p class="text-sm text-gray-500">
                {{ engine.description }}
              </p>
              <p class="text-sm text-gray-500">License: {{ engine.license }}</p>
              <p class="text-sm text-gray-500">
                {{ engine.website }}
              </p>
            </div>
            <div class="flex-shrink-0 self-center">
              <!-- Heroicon name: mini/chevron-right -->
              <svg
                class="h-5 w-5 text-gray-400 group-hover:text-gray-500"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 20 20"
                fill="currentColor"
                aria-hidden="true"
              >
                <path
                  fill-rule="evenodd"
                  d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                  clip-rule="evenodd"
                />
              </svg>
            </div>
          </div>
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
