<script setup lang="ts">
import { useWorkRequestsStore } from '../stores/work-requests';
const workRequests = useWorkRequestsStore()
</script>

<template>
  <div class="w-3/4 mx-auto my-20">
    <h1 class="text-2xl mb-8">Analysis Requests</h1>
    <div class="bg-base-200 mt-4 p-4">
      <div v-if="workRequests.requests.length" v-for="request in workRequests.requests">
        {{ request.engine.name }}
        -
        <code>{{ request.work.initialFen }}</code>
        -
        <code>{{ request.work.moves.join(' ') }}</code>
      </div>

      <div v-else>No analysis requests yet</div>
    </div>
  </div>
</template>
