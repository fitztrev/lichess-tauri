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
            {{ engine.maxThreads }} threads &bullet;
            {{ memoryToHumanReadable(engine.maxHash) }} &bullet;
            {{ engine.defaultDepth }} depth
          </p>
        </div>
      </div>
    </div>
  </a>
  <a href="#" @click.prevent="deleteEngine">Delete</a>
  <router-link :to="{ name: 'editEngine', params: { id: engine.id } }">
    Edit
  </router-link>
</template>
