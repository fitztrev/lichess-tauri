<script setup lang="ts">
import PageTitle from './PageTitle.vue'
import GettingStarted from './GettingStarted.vue'
import Chessboard from './Chessboard.vue'
import { useSettingsStore } from '../stores/settings'
import { useEnginesStore } from '../stores/engines'
import { useAnalysisStore } from '../stores/analysis'

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
        <h3 class="text-4xl">{{ analysis.evaluation }}</h3>
        <p>
          {{ analysis.uci.scoreType }}
          {{ analysis.uci.scoreValue }}
          {{ analysis.uci.scoreBound }}
        </p>
        <p>Evaluation</p>

        <h3 class="mt-2 text-xl">{{ analysis.uci.depth }}</h3>
        <p>Depth</p>

        <h3 class="mt-2 text-xl">{{ analysis.uci.seldepth }}</h3>
        <p>Sel Depth</p>

        <h3 class="mt-2 text-xl">{{ analysis.nodes }}</h3>
        <p>Nodes</p>

        <h3 class="mt-2 text-xl">{{ analysis.nps }}</h3>
        <p>Nodes/sec</p>

        <h3 class="mt-2 text-xl">{{ analysis.time }}</h3>
        <p>Time</p>

        <h3 class="mt-2 text-xl">{{ analysis.hashUsage }}</h3>
        <p>Hash Usage %</p>

        <h3 class="mt-2 text-xl">{{ analysis.uci.tbhits }}</h3>
        <p>Tablebase Hits</p>

        <h3 class="mt-8 text-lg">{{ analysis.status }}</h3>
        <p>Status</p>
      </div>
    </div>
  </template>
</template>
