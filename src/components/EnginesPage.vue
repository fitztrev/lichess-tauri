<script setup lang="ts">
import { useEnginesStore } from '../stores/engines'
import { open as openShell } from '@tauri-apps/api/shell'
import { open as openDialog } from '@tauri-apps/api/dialog'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { saveEngine } from '../utils/engine-crud'
import { RouterLink } from 'vue-router'
import { router } from '../router'
import { sysinfo } from '../utils/sysyinfo'

const engines = useEnginesStore()
const engineDirectory = ref<EngineListing[]>([])

function openContainingFolder(filepath: string) {
  let dir = filepath.substring(0, filepath.lastIndexOf('/'))
  openShell(dir)
}

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
    saveEngine({
      name: engine.name + ' ' + engine.version,
      maxThreads: maxThreads,
      maxHash: maxHash,
      defaultDepth: 25,
      variants: ['chess'],
      binaryLocation: path_to_binary,
    }).then(() => {
      router.push('/engines')
    })
  })
}

interface EngineListing {
  name: string
  description: string
  website: string
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
  <div class="w-3/4 mx-auto my-20">
    <h1 class="text-2xl mb-8">Engines</h1>

    <div class="alert shadow-lg my-20" v-if="!engines.engines.length">
      <div>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          class="stroke-current flex-shrink-0 w-6 h-6"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          ></path>
        </svg>
        <span>No engines added yet</span>
      </div>
    </div>

    <div
      class="card w-3/4 bg-base-200 shadow-xl mb-8"
      v-for="engine in engines.engines"
    >
      <div class="card-body">
        <h2 class="card-title">{{ engine.name }}</h2>
        <div>
          <span class="badge badge-primary"
            >{{ engine.maxThreads }} threads</span
          >
          <span class="badge badge-primary">{{ engine.maxHash }} MB</span>
          <span class="badge badge-primary"
            >{{ engine.defaultDepth }} max depth</span
          >
          <span v-for="variant in engine.variants">
            <span class="badge badge-accent badge-outline">{{ variant }}</span>
          </span>
        </div>
        <a
          href="#"
          class="text-sm font-mono"
          @click.prevent="openContainingFolder(engine.binaryLocation)"
        >
          {{ engine.binaryLocation }}
        </a>
      </div>
    </div>

    <h1>Add an engine</h1>

    <div
      v-for="engine in engineDirectory"
      class="card w-96 bg-base-200 shadow-xl"
      @click="addEngineFromDirectory(engine)"
    >
      <div class="card-body">
        <h2 class="card-title">{{ engine.name }} {{ engine.version }}</h2>
        <p>{{ engine.description }}</p>
        <p>License: {{ engine.license }}</p>
        <p>Website: {{ engine.website }}</p>
      </div>
    </div>

    <router-link to="/engines/custom">
      <div
        class="card w-96 bg-base-200 shadow-xl border-dashed border-2 border-gray-500"
      >
        <div class="card-body">
          <h2 class="card-title">Add a custom engine</h2>
        </div>
      </div>
    </router-link>
  </div>
</template>
