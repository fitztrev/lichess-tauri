import { invoke } from '@tauri-apps/api'
import { useSettingsStore } from '../stores/settings'

export async function loadSettingsFromDatabase() {
  let settings_from_database = await invoke<{
    lichess_host: string
    engine_host: string
    provider_secret: string
    username: string
    token: string
  }>('get_all_settings')

  let settings = useSettingsStore()
  settings.lichessHost = settings_from_database.lichess_host
  settings.engineHost = settings_from_database.engine_host
  settings.providerSecret = settings_from_database.provider_secret
  settings.username = settings_from_database.username
  settings.token = settings_from_database.token
}
