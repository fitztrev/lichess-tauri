<script setup lang="ts">
import Chessboard from './Chessboard.vue'
import GettingStarted from './GettingStarted.vue'
import PageTitle from './PageTitle.vue'
import SleepCountdown from './SleepCountdown.vue'

import { useAnalysisStore } from '../stores/analysis'
import { useEnginesStore } from '../stores/engines'
import { useSettingsStore } from '../stores/settings'

const engines = useEnginesStore()
const settings = useSettingsStore()
const analysis = useAnalysisStore()
</script>

<template>
  <GettingStarted v-if="!settings.isLoggedIn || !engines.hasEngines" />

  <template v-else>
    <PageTitle>Dashboard</PageTitle>

    <div class="page-content flex">
      <div class="flex-initial w-2/3">
        <Chessboard :fen="analysis.fen"></Chessboard>
      </div>
      <div class="flex-initial w-1/3 text-center">
        <h3 class="text-5xl font-bold">{{ analysis.evaluation }}</h3>
        <p>Evaluation</p>

        <div class="flex">
          <div class="flex-initial w-1/2">
            <h3 class="mt-2 text-3xl">{{ analysis.uci.depth }}</h3>
            <div class="text-gray-600">Depth</div>
          </div>
          <div class="flex-initial w-1/2">
            <h3 class="mt-2 text-3xl">{{ analysis.uci.seldepth }}</h3>
            <div class="text-gray-600">Sel Depth</div>
          </div>
        </div>

        <div class="flex">
          <div class="flex-initial w-1/2">
            <h3 class="mt-2 text-3xl">{{ analysis.nodes }}</h3>
            <div class="text-gray-600">Nodes</div>
          </div>
          <div class="flex-initial w-1/2">
            <h3 class="mt-2 text-3xl">{{ analysis.nps }}</h3>
            <div class="text-gray-600">Nodes/sec</div>
          </div>
        </div>

        <div class="flex">
          <div class="flex-initial w-1/2">
            <h3 class="mt-2 text-3xl">{{ analysis.time }}</h3>
            <div class="text-gray-600">Time</div>
          </div>
          <div class="flex-initial w-1/2">
            <h3 class="mt-2 text-3xl">{{ analysis.hashUsage }}</h3>
            <div class="text-gray-600">Hash Usage %</div>
          </div>
        </div>

        <h3 class="mt-8 text-lg">{{ analysis.status }}</h3>
        <SleepCountdown
          v-if="analysis.sleepDuration > 0"
          :duration="analysis.sleepDuration"
        />
      </div>
    </div>
  </template>
</template>
