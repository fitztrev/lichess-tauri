<script setup lang="ts">
import { Engine, useEnginesStore } from "../stores/engines";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/shell";
import { useSettingsStore } from "../stores/settings";
import { useUserStore } from "../stores/user";
import { useWorkRequestsStore, WorkRequest } from "../stores/work-requests";

const engines = useEnginesStore();
const settings = useSettingsStore();
const user = useUserStore();
const workRequests = useWorkRequestsStore();

async function runEngine(engine: Engine, data: WorkRequest) {
  let response = await invoke("run_engine", {
    host: settings.externalEngineHost,
    token: user.token,
    id: data.id,
    binary: engine.binaryLocation,
    fen: data.work.initialFen,
    moves: data.work.moves.join(" "),
  });
  console.log("response", response);
}

function openLichess(url: string) {
  open(`${settings.lichessHost}/${url}`);
}

async function listenForWork() {
  while (true) {
    console.log("requesting work");
    await fetch(`${settings.externalEngineHost}/api/external-engine/work`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + user.token,
      },
      body: JSON.stringify({
        providerSecret: "my-provider-secret",
      }),
    })
      .then((response) => {
        return response.json();
      })
      .then((data) => {
        workRequests.add(data);

        let engine = engines.findById(data.engine.id);

        if (engine) {
          console.log("running engine", engine, data);
          runEngine(engine, data);
        } else {
          console.log(`engine not found: ${data.engine.id}`);
        }
      })
      .catch((error) => {
        console.log("no new work detected");
      });
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}

listenForWork();
</script>

<template>
  <div class="bg-emerald-700 text-emerald-100 text-center py-2">
    <svg
      class="animate-spin inline-block -ml-1 mr-3 h-5 w-5 text-white"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      ></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      ></path>
    </svg>
    Listening for requests from the
    <a href="#" @click.prevent="openLichess('/analysis')" class="underline"
      >Analysis page</a
    >...
  </div>
</template>
