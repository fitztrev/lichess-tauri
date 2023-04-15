<script setup lang="ts">
defineProps<{
  status: 'completed' | 'current' | 'future'
}>()
</script>

<template>
  <li>
    <a href="#" class="flex items-start">
      <span
        class="relative flex h-5 w-5 flex-shrink-0 items-center justify-center"
        aria-hidden="true"
      >
        <template v-if="status === 'completed'">
          <svg
            class="h-full w-full text-indigo-600 group-hover:text-indigo-800"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
          >
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
              clip-rule="evenodd"
            />
          </svg>
        </template>
        <template v-else-if="status === 'current'">
          <span class="absolute h-4 w-4 rounded-full bg-indigo-200"></span>
          <span
            class="relative block h-2 w-2 rounded-full bg-indigo-600"
          ></span>
        </template>
        <template v-else>
          <div
            class="h-2 w-2 rounded-full bg-gray-300 group-hover:bg-gray-400"
          ></div>
        </template>
      </span>
      <span
        :class="{
          'ml-3 text-sm font-medium text-gray-500 group-hover:text-gray-900':
            status !== 'current',
          'ml-3 text-sm font-medium text-indigo-600': status === 'current',
        }"
        ><slot></slot
      ></span>
    </a>
  </li>
</template>
