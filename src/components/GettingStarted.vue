<script setup lang="ts">
import LichessLogin from './LichessLogin.vue'
import EnginesPage from './EnginesPage.vue'
import ChecklistItem from './ChecklistItem.vue'
import { useUserStore } from '../stores/user'
import { ref } from 'vue'

const user = useUserStore()

let currentStep = ref(1)

// This "Getting Started" only has 2 steps:
// 1. Login to Lichess
// 2. Add an engine
currentStep.value = user.isLoggedIn ? 2 : 1
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
            <ChecklistItem :status="currentStep === 1 ? 'current' : 'completed'"
              >Log in with Lichess</ChecklistItem
            >
            <ChecklistItem :status="currentStep === 2 ? 'current' : 'future'"
              >Add a chess engine</ChecklistItem
            >
            <ChecklistItem status="future"
              >Analyze a position on Lichess.org</ChecklistItem
            >
          </ol>
        </nav>
      </div>
    </div>

    <div class="text-center mt-12" v-if="currentStep === 1">
      <LichessLogin />
    </div>
    <div class="mt-6" v-else>
      <EnginesPage />
    </div>
  </div>
</template>
