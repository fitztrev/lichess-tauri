<script setup lang="ts">
import { LichessEngine, refreshEngineList } from '../stores/engines'
import { deleteEngineFromLichess } from '../utils/engine-crud'

import { memoryToHumanReadable } from '../utils/sysyinfo'

const props = defineProps<{
  engine: LichessEngine
}>()

function deleteEngine(): void {
  deleteEngineFromLichess(props.engine).then(() => {
    refreshEngineList()
  })
}
</script>

<template>
  <div class="px-4 py-4 sm:px-6">
    <div class="flex items-center justify-between">
      <p class="truncate font-medium text-indigo-600">
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
          {{ engine.maxThreads }} threads &bullet;
          {{ memoryToHumanReadable(engine.maxHash) }} &bullet;
          {{ engine.defaultDepth }} depth
        </p>
      </div>
    </div>
    <div class="mt-2">
      <a
        href="#"
        class="inline-flex px-2 mr-2 rounded bg-red-100 text-red-800 hover:bg-red-500 hover:text-white focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
        @click.prevent="deleteEngine"
        >Delete</a
      >
      <router-link
        :to="{ name: 'editEngine', params: { id: engine.id } }"
        class="inline-flex px-2 rounded bg-indigo-100 text-indigo-800 hover:bg-indigo-500 hover:text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
      >
        Edit
      </router-link>
    </div>
  </div>
</template>
