import { invoke } from '@tauri-apps/api'
import { useSettingsStore } from '../stores/settings'

export async function loadSettingsFromDatabase() {
  let settings_from_database = await invoke<{
    lichess_host: string
    engine_host: string
    provider_secret: string
    lichess_username: string
    lichess_token: string
  }>('get_all_settings')

  let settings = useSettingsStore()
  settings.lichessHost = settings_from_database.lichess_host
  settings.engineHost = settings_from_database.engine_host
  settings.providerSecret = settings_from_database.provider_secret
  settings.lichess_username = settings_from_database.lichess_username
  settings.lichess_token = settings_from_database.lichess_token
}

export function trimTrailingSlash(url: string) {
  return url.replace(/\/$/, '')
}

if (import.meta.vitest) {
  const { it, expect } = import.meta.vitest

  it('trims trailing slashes', () => {
    expect(trimTrailingSlash('https://lichess.org/')).toStrictEqual(
      'https://lichess.org'
    )
  })
}
