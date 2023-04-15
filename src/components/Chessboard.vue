<script setup lang="ts">
import { Chessground } from 'chessground'
import { Api } from 'chessground/api'
import { onMounted, ref, watch } from 'vue'

let board: Api | undefined
const boardElement = ref<HTMLElement | null>(null)

const props = defineProps<{
  fen: string
}>()

onMounted(() => {
  if (boardElement.value) {
    board = Chessground(boardElement.value, {
      fen: props.fen,
      viewOnly: true,
    })
  }
})

watch(
  () => props.fen,
  (fen) => {
    if (board) {
      board.set({ fen })
    }
  }
)
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
