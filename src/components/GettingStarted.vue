<script setup lang="ts">
import LichessLogin from './LichessLogin.vue'
import EnginesPage from './EnginesPage.vue'
import ChecklistItem from './ChecklistItem.vue'
import { useSettingsStore } from '../stores/settings'

const settings = useSettingsStore()
</script>

<template>
  <div class="page-content">
    <div class="bg-white p-8">
      <div class="text-center text-gray-700">
        <h1 class="text-2xl">Welcome to the Lichess Local Engine!</h1>
        <p class="my-2">
          This app will let you use analyze positions with a stronger chess
          engine than you can run in your browser.
        </p>
      </div>
      <div class="py-12 px-4 sm:px-6 lg:px-8">
        <nav class="flex justify-center" aria-label="Progress">
          <ol role="list" class="space-y-6">
            <ChecklistItem
              :status="!settings.isLoggedIn ? 'current' : 'completed'"
              >Log in with Lichess</ChecklistItem
            >
            <ChecklistItem :status="settings.isLoggedIn ? 'current' : 'future'"
              >Add a chess engine</ChecklistItem
            >
            <ChecklistItem status="future"
              >Analyze a position on Lichess.org</ChecklistItem
            >
          </ol>
        </nav>
      </div>
    </div>

    <div class="text-center mt-12" v-if="!settings.isLoggedIn">
      <LichessLogin />
    </div>
    <div class="mt-6" v-else>
      <EnginesPage />
    </div>
  </div>
</template>
