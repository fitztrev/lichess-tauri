<script setup lang="ts">
import Chessboard from './Chessboard.vue'
import GettingStarted from './GettingStarted.vue'
import { open } from '@tauri-apps/api/shell'

import { useAnalysisStore } from '../stores/analysis'
import { useEnginesStore } from '../stores/engines'
import { useSettingsStore } from '../stores/settings'

const engines = useEnginesStore()
const settings = useSettingsStore()
const analysis = useAnalysisStore()

function openAnalysisPageOnLichess() {
  let url = settings.lichessHost + '/analysis'

  if (engines.engines.length > 0) {
    url += '?engine=' + engines.engines[0].id
  }

  open(url)
}
</script>

<template>
  <GettingStarted v-if="!settings.isLoggedIn || !engines.hasEngines" />

  <template v-else>
    <div class="page-content flex">
      <div class="flex-initial w-2/3">
        <Chessboard :fen="analysis.fen"></Chessboard>
        <div class="text-center text-gray-500 mt-2">
          Open
          <a
            href="#"
            @click.prevent="openAnalysisPageOnLichess()"
            class="underline"
            >Analysis page</a
          >
        </div>
      </div>
      <div class="flex-initial w-1/3 text-center">
        <div class="mt-8 mb-6" v-if="analysis.evaluation">
          <h3 class="text-5xl font-bold">{{ analysis.evaluation }}</h3>
          <p>Evaluation</p>
        </div>

        <div class="flex">
          <div class="flex-initial w-1/2" v-if="analysis.uci.depth">
            <h3 class="mt-2 text-3xl">{{ analysis.uci.depth }}</h3>
            <div class="text-gray-600">Depth</div>
          </div>
          <div class="flex-initial w-1/2" v-if="analysis.uci.seldepth">
            <h3 class="mt-2 text-3xl">{{ analysis.uci.seldepth }}</h3>
            <div class="text-gray-600">Sel Depth</div>
          </div>
        </div>

        <div class="flex">
          <div class="flex-initial w-1/2" v-if="analysis.nodes">
            <h3 class="mt-2 text-3xl">{{ analysis.nodes }}</h3>
            <div class="text-gray-600">Nodes</div>
          </div>
          <div class="flex-initial w-1/2" v-if="analysis.nps">
            <h3 class="mt-2 text-3xl">{{ analysis.nps }}</h3>
            <div class="text-gray-600">Nodes/sec</div>
          </div>
        </div>

        <div class="flex">
          <div class="flex-initial w-1/2" v-if="analysis.time">
            <h3 class="mt-2 text-3xl">{{ analysis.time }}</h3>
            <div class="text-gray-600">Time</div>
          </div>
          <div class="flex-initial w-1/2" v-if="analysis.hashUsage">
            <h3 class="mt-2 text-3xl">{{ analysis.hashUsage }}</h3>
            <div class="text-gray-600">Hash Usage %</div>
          </div>
        </div>
      </div>
    </div>
  </template>
</template>
