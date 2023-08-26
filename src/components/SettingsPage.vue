<script setup lang="ts">
import { useSettingsStore } from '../stores/settings'
import LichessLogin from './LichessLogin.vue'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'
import { loadSettingsFromDatabase, trimTrailingSlash } from '../utils/settings'
import { getVersion } from '@tauri-apps/api/app'

const settings = useSettingsStore()

const inputLichessHost = ref(settings.lichessHost)
const inputEngineHost = ref(settings.engineHost)

const appDataDir = ref('')
invoke<string>('get_app_data_dir').then((dir) => {
  appDataDir.value = dir
})

const appVersion = ref('')
getVersion().then((version) => {
  appVersion.value = version
})

async function save() {
  await invoke('update_setting', {
    key: 'lichess_host',
    value: trimTrailingSlash(inputLichessHost.value),
  })
  await invoke('update_setting', {
    key: 'engine_host',
    value: trimTrailingSlash(inputEngineHost.value),
  })

  await loadSettingsFromDatabase()

  inputLichessHost.value = settings.lichessHost
  inputEngineHost.value = settings.engineHost
}

async function openPath(path: string) {
  await invoke('open_path', { path })
}
</script>

<template>
  <div class="page-content">
    <div class="bg-white shadow sm:rounded-lg my-8">
      <div class="px-4 py-5 sm:p-6" v-if="settings.isLoggedIn">
        <h3 class="text-lg font-medium leading-6 text-gray-900">
          Logged in as <strong>{{ settings.lichess_username }}</strong>
        </h3>
        <div class="mt-5">
          <button
            @click="settings.logout"
            type="button"
            class="inline-flex items-center justify-center rounded-md border border-transparent bg-red-100 px-4 py-2 font-medium text-red-700 hover:bg-red-500 hover:text-white focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 sm:text-sm"
          >
            Logout
          </button>
        </div>
      </div>
      <div class="px-4 py-5 sm:p-6" v-else>
        <h3 class="text-lg font-medium leading-6 text-gray-900">
          Log in with Lichess
        </h3>
        <div class="mt-5">
          <LichessLogin />
        </div>
      </div>
    </div>

    <form class="space-y-8 divide-y divide-gray-200">
      <div class="space-y-8 divide-y divide-gray-200 sm:space-y-5">
        <div class="space-y-6 sm:space-y-5">
          <div>
            <h3 class="text-lg font-medium leading-6 text-gray-900">
              Advanced
            </h3>
            <p class="mt-1 max-w-2xl text-sm text-gray-500">
              For development and testing purposes. These settings change which
              server the app communicates with.
            </p>
          </div>

          <div class="space-y-6 sm:space-y-5">
            <div
              class="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5"
            >
              <label
                for="lichessHost"
                class="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
                >Lichess Host</label
              >
              <div class="mt-1 sm:col-span-2 sm:mt-0">
                <input
                  v-model="inputLichessHost"
                  id="lichessHost"
                  type="text"
                  class="block w-full max-w-lg rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                />
              </div>
            </div>
            <div
              class="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5"
            >
              <label
                for="engineHost"
                class="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
                >External Engine Host</label
              >
              <div class="mt-1 sm:col-span-2 sm:mt-0">
                <input
                  v-model="inputEngineHost"
                  id="engineHost"
                  type="text"
                  class="block w-full max-w-lg rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="pt-5">
        <div class="flex justify-end">
          <router-link
            to="/"
            class="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
            >Cancel</router-link
          >
          <button
            @click="save"
            type="button"
            class="ml-3 inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            Save
          </button>
        </div>
      </div>
    </form>

    <h3 class="text-lg font-medium leading-6 text-gray-900 mt-8">About</h3>
    <p class="mt-1 max-w-2xl text-sm text-gray-700">
      You are using version v<strong>{{ appVersion }}</strong>
      <br />
      Local database and engines are stored at
      <a href="#" @click.prevent="openPath(appDataDir)" class="underline">{{
        appDataDir
      }}</a>
      <br />
      See the source and contribute to this
      <a
        href="#"
        @click.prevent="openPath('https://github.com/fitztrev/lichess-tauri')"
        class="underline"
        >app on GitHub</a
      >
    </p>
  </div>
</template>
