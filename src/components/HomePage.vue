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
      <Chessboard :fen="analysis.fen"></Chessboard>
      initial fen: {{ analysis.request.work?.initialFen }}
      <br />
      moves: {{ analysis.request.work?.moves }}
      <br />
      computed fen: {{ analysis.fen }}

      <h3 class="font-bold m-3">Status</h3>
      <p>{{ analysis.status }}</p>

      <table>
        <tr>
          <td>Evaluation</td>
          <td>
            {{ analysis.uci.score?.type }} {{ analysis.uci.score?.value }}
          </td>
        </tr>
        <tr>
          <td>Depth</td>
          <td>{{ analysis.uci.depth }}</td>
        </tr>
        <tr>
          <td>Sel Depth</td>
          <td>{{ analysis.uci.seldepth }}</td>
        </tr>
        <tr>
          <td>Nodes</td>
          <td>{{ analysis.nodes }}</td>
        </tr>
        <tr>
          <td>Nodes/sec</td>
          <td>{{ analysis.nps }}</td>
        </tr>
        <tr>
          <td>Hashfull</td>
          <td>{{ analysis.uci.hashfull }}</td>
        </tr>
        <tr>
          <td>Tablebase Hits</td>
          <td>{{ analysis.uci.tbhits }}</td>
        </tr>
        <tr>
          <td>Best Move</td>
          <td>{{ analysis.uci.bestmove }}</td>
        </tr>
      </table>

      <pre>{{ analysis.uci }}</pre>
    </div>
  </template>
</template>
