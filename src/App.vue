<script setup lang="ts">
import LocalEngine from './components/LocalEngine.vue'
import { useEnginesStore } from './stores/engines';
import { useUserStore } from './stores/user'

const user = useUserStore()
const engines = useEnginesStore()

</script>

<template>
  <div>
    <div class="hidden md:fixed md:inset-y-0 md:flex md:w-64 md:flex-col">
      <div class="flex min-h-0 flex-1 flex-col bg-gray-800">
        <div class="flex flex-1 flex-col overflow-y-auto pt-5 pb-4">
          <div class="flex flex-shrink-0 items-center px-4">
            <img
              class="h-8 w-auto"
              src="./assets/lichess-white.svg"
              alt="Lichess logo"
            />
          </div>
          <nav class="mt-5 flex-1 space-y-1 px-2">
            <router-link to="/" custom v-slot="{ href, isActive, navigate }"
              ><a
                :href="href"
                @click="navigate"
                class="group flex items-center px-2 py-2 text-sm font-medium rounded-md"
                :class="{
                  'bg-gray-900 text-white': isActive,
                  'text-gray-300 hover:bg-gray-700 hover:text-white': !isActive,
                }"
              >
                <svg
                  class="mr-3 flex-shrink-0 h-6 w-6"
                  :class="{
                    'text-gray-300': isActive,
                    'text-gray-400 group-hover:text-gray-300': !isActive,
                  }"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
                  />
                </svg>
                Dashboard</a
              ></router-link
            >

            <router-link
              v-if="user.isLoggedIn"
              to="/engines"
              custom
              v-slot="{ href, isActive, navigate }"
              ><a
                :href="href"
                @click="navigate"
                class="group flex items-center px-2 py-2 text-sm font-medium rounded-md"
                :class="{
                  'bg-gray-900 text-white': isActive,
                  'text-gray-300 hover:bg-gray-700 hover:text-white': !isActive,
                }"
              >
                <svg
                  class="mr-3 flex-shrink-0 h-6 w-6"
                  :class="{
                    'text-gray-300': isActive,
                    'text-gray-400 group-hover:text-gray-300': !isActive,
                  }"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M10.5 6h9.75M10.5 6a1.5 1.5 0 11-3 0m3 0a1.5 1.5 0 10-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-9.75 0h9.75"
                  />
                </svg>
                Engines</a
              ></router-link
            >

            <router-link
              to="/settings"
              custom
              v-slot="{ href, isActive, navigate }"
              ><a
                :href="href"
                @click="navigate"
                class="group flex items-center px-2 py-2 text-sm font-medium rounded-md"
                :class="{
                  'bg-gray-900 text-white': isActive,
                  'text-gray-300 hover:bg-gray-700 hover:text-white': !isActive,
                }"
              >
                <svg
                  class="mr-3 flex-shrink-0 h-6 w-6"
                  :class="{
                    'text-gray-300': isActive,
                    'text-gray-400 group-hover:text-gray-300': !isActive,
                  }"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z"
                  />
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                  />
                </svg>
                Settings</a
              ></router-link
            >
          </nav>
        </div>
      </div>
    </div>
    <div class="flex flex-1 flex-col md:pl-64">
      <div
        class="sticky top-0 z-10 bg-gray-100 pl-1 pt-1 sm:pl-3 sm:pt-3 md:hidden"
      >
        <button
          type="button"
          class="-ml-0.5 -mt-0.5 inline-flex h-12 w-12 items-center justify-center rounded-md text-gray-500 hover:text-gray-900 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
        >
          <span class="sr-only">Open sidebar</span>
          <!-- Heroicon name: outline/bars-3 -->
          <svg
            class="h-6 w-6"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            aria-hidden="true"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
            />
          </svg>
        </button>
      </div>
      <main class="flex-1">
        <LocalEngine v-if="user.isLoggedIn && engines.engines.length > 0" />
        <div class="py-6">
          <router-view />
        </div>
      </main>
    </div>
  </div>
</template>
