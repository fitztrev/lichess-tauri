<script setup lang="ts">
import LichessLogin from './components/LichessLogin.vue'
import LocalEngine from './components/LocalEngine.vue'
import { useUserStore } from './stores/user'

const user = useUserStore()
</script>

<template>
  <div class="navbar bg-base-300">
    <div class="container mx-auto">
      <div class="flex-1">
        <router-link to="/" class="btn btn-ghost normal-case text-xl">
          <img
            src="./assets/lichess-white.svg"
            class="logo w-6"
            alt="Lichess logo"
          />
        </router-link>
      </div>
      <div class="flex-none">
        <ul class="menu menu-horizontal p-0">
          <li><router-link to="/">Dashboard</router-link></li>
          <li><router-link to="/engines">Engines</router-link></li>
          <li><router-link to="/settings">Settings</router-link></li>
          <li><LichessLogin /></li>
        </ul>
      </div>
    </div>
  </div>
  <LocalEngine v-if="user.username" />

  <div class="container mx-auto p-4">
    <router-view />
  </div>
</template>
