<script setup lang="ts">
import { useEnginesStore } from "../stores/engines";
import { open } from "@tauri-apps/api/shell";

const engines = useEnginesStore();

function openContainingFolder(filepath: string) {
  let dir = filepath.substring(0, filepath.lastIndexOf("/"));
  open(dir);
}
</script>

<template>
  <div class="w-3/4 mx-auto my-20">
    <router-link
      to="/engines/new"
      class="btn btn-circle btn-accent float-right"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="w-6 h-6"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 4.5v15m7.5-7.5h-15"
        />
      </svg>
    </router-link>

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
  </div>
</template>
