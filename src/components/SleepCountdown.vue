<script setup lang="ts">
import { onMounted, ref } from 'vue'

const props = defineProps<{
  duration: number
}>()

const remaining_secs = ref(props.duration)

onMounted(() => {
  setInterval(() => {
    remaining_secs.value -= 1
  }, 1000)
})
</script>

<template>Sleeping for {{ remaining_secs }} seconds</template>
