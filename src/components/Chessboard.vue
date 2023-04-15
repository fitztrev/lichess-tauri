<script setup lang="ts">
import { Chessground } from 'chessground'
import { onMounted, ref } from 'vue'

const boardElement = ref<HTMLElement | null>(null)

onMounted(() => {
  if (boardElement.value) {
    Chessground(boardElement.value, {
      viewOnly: true,
    })
  }
})
</script>

<style>
.chessboard {
  width: 480px;
  height: 480px;
  opacity: 0.5;
}

.chessboard cg-board {
  background-color: #e1e6ea;
}
</style>

<template>
  <div ref="boardElement" class="chessboard"></div>
</template>
