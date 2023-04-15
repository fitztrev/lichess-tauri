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

    <div class="page-content">
      <h3 class="font-bold m-3">Status</h3>
      <p>{{ analysis.status }}</p>

      <Chessboard :fen="analysis.fen"></Chessboard>

      <ul>
        <li>Evaluation: {{ analysis.evaluation }}</li>
        <li>
          Score:
          {{ analysis.uci.score?.type }}
          {{ analysis.uci.score?.value }}
          {{ analysis.uci.score?.bound }}
        </li>
        <li>Depth: {{ analysis.uci.depth }}</li>
        <li>Sel Depth: {{ analysis.uci.seldepth }}</li>
        <li>Nodes: {{ analysis.nodes }}</li>
        <li>Nodes/sec: {{ analysis.nps }}</li>
        <li>Hashfull: {{ analysis.uci.hashfull }}</li>
        <li>Tablebase Hits: {{ analysis.uci.tbhits }}</li>
        <li>Best Move: {{ analysis.uci.bestmove }}</li>
      </ul>

      <pre class="mt-12">{{ analysis.uci }}</pre>
    </div>
  </template>
</template>
