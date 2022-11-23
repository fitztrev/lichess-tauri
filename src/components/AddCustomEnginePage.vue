<script setup lang="ts">
import { open } from '@tauri-apps/api/dialog'
import { Ref, ref } from 'vue'
import { router } from '../router'
import { Engine } from '../stores/engines'
import { saveEngine } from '../utils/engine-crud'
import { sysinfo } from '../utils/sysyinfo'
import PageTitle from './PageTitle.vue'

const name = ref('')
const maxThreads = ref(1)
const maxHash = ref(16)
const defaultDepth = ref(25)
const binaryLocation: Ref<any> = ref('')

const maxHashOptions = ref<number[]>([])
const maxThreadOptions = ref<number[]>([])

sysinfo().then((data) => {
  let memoryLimit = (data.total_memory / 1024 / 1024) * 0.7 // up to 70% of total memory
  for (let i = 16; i <= memoryLimit; i *= 2) {
    maxHashOptions.value.push(i)
  }
  maxHash.value = maxHashOptions.value.slice(-1)[0]

  maxThreadOptions.value = Array.from(
    { length: data.cpus_len },
    (_, i) => i + 1
  )
  maxThreads.value = data.cpus_len
})

function selectEngineFile() {
  open({}).then((data) => {
    binaryLocation.value = data
  })
}

function submit() {
  let engine: Engine = {
    name: name.value,
    maxThreads: maxThreads.value,
    maxHash: maxHash.value,
    defaultDepth: defaultDepth.value,
    variants: ['chess'],
    binaryLocation: binaryLocation.value,
  }

  saveEngine(engine).then(() => {
    // router.push('/engines')
  })
}
</script>

<template>
  <PageTitle>Add Custom Engine</PageTitle>

  <div class="page-content">
    <form class="space-y-8 divide-y divide-gray-200">
      <div class="space-y-8 divide-y divide-gray-200 sm:space-y-5">
        <div class="space-y-6 pt-8 sm:space-y-5 sm:pt-10">
          <div>
            <h3 class="text-lg font-medium leading-6 text-gray-900">
              Engine Information
            </h3>
            <p class="mt-1 max-w-2xl text-sm text-gray-500">
              Add a UCI-compatible engine
            </p>
          </div>
          <div class="space-y-6 sm:space-y-5">
            <div
              class="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5"
            >
              <label
                for="name"
                class="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
                >Name</label
              >
              <div class="mt-1 sm:col-span-2 sm:mt-0">
                <input
                  type="text"
                  id="name"
                  autocomplete="given-name"
                  class="block w-full max-w-lg rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:max-w-xs sm:text-sm"
                />
              </div>
            </div>

            <div
              class="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5"
            >
              <label
                for="country"
                class="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
                >Max Threads</label
              >
              <div class="mt-1 sm:col-span-2 sm:mt-0">
                <select
                  v-model="maxThreads"
                  class="block w-full max-w-lg rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:max-w-xs sm:text-sm"
                >
                  <option
                    v-for="option in maxThreadOptions"
                    :key="option"
                    :value="option"
                  >
                    {{ option }}
                  </option>
                </select>
              </div>
            </div>
            <div
              class="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5"
            >
              <label
                for="country"
                class="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
                >Max Hash</label
              >
              <div class="mt-1 sm:col-span-2 sm:mt-0">
                <select
                  v-model="maxHash"
                  class="block w-full max-w-lg rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:max-w-xs sm:text-sm"
                >
                  <option
                    v-for="option in maxHashOptions"
                    :key="option"
                    :value="option"
                  >
                    {{ option }} MB
                  </option>
                </select>
              </div>
            </div>
            <div
              class="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5"
            >
              <label
                for="country"
                class="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
                >Default Depth</label
              >
              <div class="mt-1 sm:col-span-2 sm:mt-0">
                <select
                  v-model="defaultDepth"
                  class="block w-full max-w-lg rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:max-w-xs sm:text-sm"
                >
                  <option v-for="n in 50" :value="n">
                    {{ n }}
                  </option>
                </select>
              </div>
            </div>

            <div
              class="sm:grid sm:grid-cols-3 sm:items-center sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5"
            >
              <label for="photo" class="block text-sm font-medium text-gray-700"
                >Binary</label
              >
              <div class="mt-1 sm:col-span-2 sm:mt-0">
                <div class="flex items-center">
                  <button
                    type="button"
                    @click="selectEngineFile"
                    class="rounded-md border border-gray-300 bg-white py-2 px-3 text-sm font-medium leading-4 text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                  >
                    Select
                  </button>
                </div>
                <p class="mt-2 text-sm text-gray-500">
                  {{ binaryLocation }}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="pt-5">
        <div class="flex justify-end">
          <button
            type="button"
            class="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            Cancel
          </button>
          <button
            @click="submit"
            type="button"
            class="ml-3 inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            Save
          </button>
        </div>
      </div>
    </form>
  </div>
</template>
