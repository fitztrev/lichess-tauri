<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function runEngine(data: any) {
  greetMsg.value = await invoke("run_engine", { id: data.id, fen: data.work.initialFen, moves: data.work.moves.join(' ') });
}

let analysisRequests: Ref<any[]> = ref([])

function listenForWork() {
  fetch("https://9666-lichessorg-lilagitpod-ewnvbqhjl6a.ws-us73.gitpod.io/api/external-engine/work", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      // "Authorization": "Bearer " + localStorage.getItem("token")
      "Authorization": "Bearer lip_vSMsPHtKznbEHZS8rg7o"
    },
    body: JSON.stringify({
      "providerSecret": "my-provider-secret",
    }),
  }).then((response) => {
    return response.json();
  }).then((data) => {
    analysisRequests.value.push(data)
    console.log(data);

    runEngine(data);

    listenForWork();
  });
}

listenForWork();
</script>

<template>
  <h2>Analysis Requests Debug</h2>
  <pre>{{ analysisRequests.map(function(request) {
    return JSON.stringify({
      fen: request.work.initialFen,
      moves: request.work.moves
    })
  }) }}</pre>
</template>
