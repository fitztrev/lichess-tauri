import { ref } from 'vue'
import pkceChallenge from 'pkce-challenge'

import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/shell'

import { useSettingsStore } from '../stores/settings'
import { invoke } from '@tauri-apps/api'

const clientId = 'lichess-tauri'
const tempServerUrl = ref('')
const challenge = pkceChallenge(128)

export function registerOauthHandlers(): void {
  listen('server_started', (data: { payload: number }) => {
    console.log('server started', data)
    tempServerUrl.value = `http://localhost:${data.payload}/`

    const settings = useSettingsStore()

    let url =
      `${settings.lichessHost}/oauth?` +
      new URLSearchParams({
        response_type: 'code',
        client_id: clientId,
        redirect_uri: tempServerUrl.value,
        code_challenge_method: 'S256',
        code_challenge: challenge.code_challenge,
        scope: ['engine:read', 'engine:write'].join(' '),
      }).toString()

    open(url)
  })

  listen('returning_from_lichess', (data: { payload: string }) => {
    let url = new URL(data.payload)
    let code = url.searchParams.get('code') || ''

    const settings = useSettingsStore()

    fetch(`${settings.lichessHost}/api/token`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: `grant_type=authorization_code&client_id=${clientId}&code=${code}&redirect_uri=${tempServerUrl.value}&code_verifier=${challenge.code_verifier}`,
    })
      .then((response) => response.json())
      .then(async (data) => {
        settings.token = data.access_token

        await invoke('update_setting', { key: 'token', value: data.access_token })

        fetch(`${settings.lichessHost}/api/account`, {
          headers: {
            Authorization: `Bearer ${settings.token}`,
          },
        })
          .then((response) => response.json())
          .then(async (data) => {
            settings.username = data.username

            await invoke('update_setting', { key: 'username', value: data.username })
          })
      })
  })
}
