<script setup lang="ts">
import { open } from '@tauri-apps/api/dialog'
import { Ref, ref } from 'vue'
import { router } from '../router'
import { Engine } from '../stores/engines'
import { saveEngine } from '../utils/engine-crud'
import { sysinfo } from '../utils/sysyinfo'

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
    router.push('/engines')
  })
}
</script>

<template>
  <div class="w-3/4 mx-auto my-20">
    <h1 class="text-2xl mb-8">Add Custom Engine</h1>

    <div class="form-control mb-4">
      <label class="label">
        <span class="label-text">Name</span>
      </label>
      <input type="text" class="input input-bordered" v-model="name" />
    </div>

    <div class="form-control mb-4">
      <label class="label">
        <span class="label-text">Max Threads</span>
      </label>
      <select class="select select-bordered" v-model="maxThreads">
        <option
          v-for="option in maxThreadOptions"
          :key="option"
          :value="option"
        >
          {{ option }}
        </option>
      </select>
    </div>

    <div class="form-control mb-4">
      <label class="label">
        <span class="label-text">Max Hash</span>
      </label>
      <select class="select select-bordered" v-model="maxHash">
        <option v-for="option in maxHashOptions" :key="option" :value="option">
          {{ option }} MB
        </option>
      </select>
    </div>

    <div class="form-control mb-4">
      <label class="label">
        <span class="label-text">Default Depth</span>
      </label>
      <input type="text" class="input input-bordered" v-model="defaultDepth" />
    </div>

    <div class="form-control mb-4">
      <label class="label">
        <span class="label-text">Binary</span>
      </label>
      <button type="button" class="btn btn-wide" @click="selectEngineFile">
        Select local file
      </button>
      {{ binaryLocation }}
    </div>

    <button class="btn btn-accent" @click="submit">Save</button>
  </div>
</template>
