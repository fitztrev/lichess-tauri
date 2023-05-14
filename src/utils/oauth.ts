import { ref } from 'vue'
import pkceChallenge from 'pkce-challenge'

import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/shell'

import { useSettingsStore } from '../stores/settings'
import { invoke } from '@tauri-apps/api'

const clientId = 'lichess-tauri'
const tempServerUrl = ref('')
const challenge = ref({ code_challenge: '', code_verifier: '' })

export function registerOauthHandlers(): void {
  listen('server_started', async (data: { payload: number }) => {
    tempServerUrl.value = `http://localhost:${data.payload}/`

    const settings = useSettingsStore()

    challenge.value = await pkceChallenge(128)

    let url =
      `${settings.lichessHost}/oauth?` +
      new URLSearchParams({
        response_type: 'code',
        client_id: clientId,
        redirect_uri: tempServerUrl.value,
        code_challenge_method: 'S256',
        code_challenge: challenge.value.code_challenge,
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
      body: `grant_type=authorization_code&client_id=${clientId}&code=${code}&redirect_uri=${tempServerUrl.value}&code_verifier=${challenge.value.code_verifier}`,
    })
      .then((response) => response.json())
      .then(async (data) => {
        settings.lichess_token = data.access_token

        await invoke('update_setting', {
          key: 'lichess_token',
          value: data.access_token,
        })

        fetch(`${settings.lichessHost}/api/account`, {
          headers: {
            Authorization: `Bearer ${settings.lichess_token}`,
          },
        })
          .then((response) => response.json())
          .then(async (data) => {
            settings.lichess_username = data.username

            await invoke('update_setting', {
              key: 'lichess_username',
              value: data.username,
            })
          })
      })
  })
}
