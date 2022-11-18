<script setup lang="ts">
import { Ref, ref } from "vue";
import { getName, getTauriVersion, getVersion } from "@tauri-apps/api/app";
import { platform, version, type, arch } from "@tauri-apps/api/os";
import { invoke } from "@tauri-apps/api/tauri";

const name = ref("");
getName().then((data) => {
  name.value = data;
});

const tauriVersion = ref("");
getTauriVersion().then((data) => {
  tauriVersion.value = data;
});

const appVersion = ref("");
getVersion().then((data) => {
  appVersion.value = data;
});

const osPlatform = ref("");
platform().then((data) => {
  osPlatform.value = data;
});

const osVersion = ref("");
version().then((data) => {
  osVersion.value = data;
});

const os = ref("");
type().then((data) => {
  os.value = data;
});

const osArch = ref("");
arch().then((data) => {
  osArch.value = data;
});

const sysinfo = ref({});
const numCores = ref(0);
const maxHashOptions = ref<number[]>([]);

invoke<{ total_memory: number; "cpus.len": number }>("get_sysinfo").then(
  (data) => {
    sysinfo.value = data;

    let memory70percent = (data.total_memory / 1024 / 1024) * 0.7; // up to 70% of total memory
    for (let i = 16; i <= memory70percent; i *= 2) {
      maxHashOptions.value.push(i);
    }

    numCores.value = data["cpus.len"];
  }
);
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
  <h3>Calculated:</h3>
  <pre>{{ { numCores, maxHashOptions } }}</pre>
</template>
