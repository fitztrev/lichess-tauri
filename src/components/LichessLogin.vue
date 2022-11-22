<script setup lang="ts">
import { ref } from 'vue'
import pkceChallenge from 'pkce-challenge'

import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/shell'

import { useUserStore } from '../stores/user'
import { useSettingsStore } from '../stores/settings'

const settings = useSettingsStore()
const user = useUserStore()

const clientId = 'lichess-tauri'
const tempServerUrl = ref('')
const challenge = pkceChallenge(128)

async function loginViaOauth() {
  await invoke('start_oauth_server')
}

listen('server_started', (data: { payload: number }) => {
  tempServerUrl.value = `http://localhost:${data.payload}/`
  openLichessLogin(tempServerUrl.value)
})

listen('returning_from_lichess', (data: { payload: string }) => {
  let url = new URL(data.payload)
  let code = url.searchParams.get('code') || ''

  fetch(`${settings.lichessHost}/api/token`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: `grant_type=authorization_code&client_id=${clientId}&code=${code}&redirect_uri=${tempServerUrl.value}&code_verifier=${challenge.code_verifier}`,
  })
    .then((response) => response.json())
    .then(async (data) => {
      user.token = data.access_token

      let profile = await getProfile()
      user.username = profile.username
    })
})

function openLichessLogin(redirect_uri: string) {
  let url =
    `${settings.lichessHost}/oauth?` +
    new URLSearchParams({
      response_type: 'code',
      client_id: 'lichess-tauri',
      redirect_uri: redirect_uri,
      code_challenge_method: 'S256',
      code_challenge: challenge.code_challenge,
      scope: ['engine:read', 'engine:write'].join(' '),
    }).toString()

  open(url)
}

async function getProfile() {
  const res = await fetch(`${settings.lichessHost}/api/account`, {
    headers: {
      Authorization: `Bearer ${user.token}`,
    },
  })

  return await res.json()
}

function logout() {
  user.destroy()
}
</script>

<template>
  <template v-if="user.username">
    {{ user.username }}
  </template>
  <template v-else>
    <a @click="loginViaOauth">Login</a>
  </template>
</template>
