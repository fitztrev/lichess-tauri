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
      <p class="truncate text-sm font-medium text-indigo-600">
        {{ engine.name }}
      </p>
      <div class="ml-2 flex flex-shrink-0">
        <div>
          <a href="#" @click.prevent="deleteEngine">Delete</a>
          <router-link :to="{ name: 'editEngine', params: { id: engine.id } }">
            Edit
          </router-link>
        </div>
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
</template>
