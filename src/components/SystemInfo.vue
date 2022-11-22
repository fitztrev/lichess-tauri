<script setup lang="ts">
import { Ref, ref } from 'vue'
import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app'
import { platform, version, type, arch } from '@tauri-apps/api/os'
import { invoke } from '@tauri-apps/api/tauri'

const name = ref('')
getName().then((data) => {
  name.value = data
})

const tauriVersion = ref('')
getTauriVersion().then((data) => {
  tauriVersion.value = data
})

const appVersion = ref('')
getVersion().then((data) => {
  appVersion.value = data
})

const osPlatform = ref('')
platform().then((data) => {
  osPlatform.value = data
})

const osVersion = ref('')
version().then((data) => {
  osVersion.value = data
})

const os = ref('')
type().then((data) => {
  os.value = data
})

const osArch = ref('')
arch().then((data) => {
  osArch.value = data
})

const sysinfo = ref({})

invoke<{}>('get_sysinfo').then((data) => {
  sysinfo.value = data
})
</script>

<template>
  <h3>From Tauri API:</h3>
  <pre>
    Tauri v{{ tauriVersion }}
    App name: {{ name }}
    App version: {{ appVersion }}
    OS: {{ os }}
    OS version: {{ osVersion }}
    OS platform: {{ osPlatform }}
    OS arch: {{ osArch }}
  </pre>
  <h3>From sysinfo crate:</h3>
  <pre>{{ sysinfo }}</pre>
</template>
