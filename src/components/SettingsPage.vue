<script setup lang="ts">
import SystemInfo from './SystemInfo.vue'
import { useSettingsStore } from '../stores/settings'
import PageTitle from './PageTitle.vue'
import { useUserStore } from '../stores/user'
import LichessLogin from './LichessLogin.vue'

const settings = useSettingsStore()
const user = useUserStore()

function logout() {
  user.destroy()
}
</script>

<template>
  <PageTitle>Settings</PageTitle>

  <div class="page-content">
    <!-- <h1 class="text-2xl my-8">Debug Info</h1>
    <SystemInfo /> -->

    <div class="bg-white shadow sm:rounded-lg my-8">
      <div class="px-4 py-5 sm:p-6" v-if="user.isLoggedIn">
        <h3 class="text-lg font-medium leading-6 text-gray-900">
          Logged in as <strong>{{ user.username }}</strong>
        </h3>
        <div class="mt-5">
          <button
            @click="logout"
            type="button"
            class="inline-flex items-center justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 font-medium text-blue-700 hover:bg-blue-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 sm:text-sm"
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
                  v-model="settings.lichessHost"
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
                for="externalEngineHost"
                class="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
                >Lichess Host</label
              >
              <div class="mt-1 sm:col-span-2 sm:mt-0">
                <input
                  v-model="settings.externalEngineHost"
                  id="externalEngineHost"
                  type="text"
                  class="block w-full max-w-lg rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </form>
  </div>
</template>
