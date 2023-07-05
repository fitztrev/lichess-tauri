import { invoke } from '@tauri-apps/api'
import { useSettingsStore } from '../stores/settings'
import { UciOption } from './types'

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

export function substituteComputedValues(
  options: UciOption[],
  substitutions: Map<string, string>
): UciOption[] {
  let substituted_options = options.map((option) => {
    let substituted_value = option.value
    substitutions.forEach((substitution_value, substitution_key) => {
      substituted_value = substituted_value.replace(
        substitution_key,
        substitution_value
      )
    })
    return { name: option.name, value: substituted_value }
  })
  return substituted_options
}

if (import.meta.vitest) {
  const { it, expect } = import.meta.vitest

  it('substitutes computed values', () => {
    let substitutions = new Map()
    substitutions.set('%HASH%', '1024')
    substitutions.set('%THREADS%', '8')

    expect(
      substituteComputedValues(
        [
          { name: 'Hash', value: '%HASH%' },
          { name: 'Threads', value: '%THREADS%' },
        ],
        substitutions
      )
    ).toStrictEqual([
      { name: 'Hash', value: '1024' },
      { name: 'Threads', value: '8' },
    ])
  })

  it('trims trailing slashes', () => {
    expect(trimTrailingSlash('https://lichess.org/')).toStrictEqual(
      'https://lichess.org'
    )
  })
}
