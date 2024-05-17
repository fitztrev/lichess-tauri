import { defineStore } from 'pinia'
import { useSettingsStore } from './settings'

type Variant =
  | 'chess'
  | 'crazyhouse'
  | 'antichess'
  | 'atomic'
  | 'horde'
  | 'kingofthehill'
  | 'racingkings'
  | '3check'

export type NewEngine = {
  name: string
  maxThreads: number
  maxHash: number
  variants: Variant[]
}

export type LichessEngine = {
  id: string
  name: string
  userId: string
  maxThreads: number
  maxHash: number
  //defaultDepth?: number;
  variants: string[]
  providerData: string
  clientSecret: string
}

export const useEnginesStore = defineStore('engines', {
  state: () => {
    return {
      engines: [] as LichessEngine[],
    }
  },
  getters: {
    hasEngines(): boolean {
      return this.engines.length > 0
    },
  },
  actions: {},
})

async function getUserEnginesFromLichess(): Promise<LichessEngine[]> {
  const settings = useSettingsStore()
  let url = `${settings.lichessHost}/api/external-engine`
  return await fetch(url, {
    headers: {
      Authorization: `Bearer ${settings.lichess_token}`,
    },
  }).then<LichessEngine[]>(async (response) => {
    if (response.status === 401) {
      await settings.logout()
      throw new Error('Unauthorized')
    }
    return response.json()
  })
}

export function refreshEngineList(): void {
  getUserEnginesFromLichess().then((data) => {
    useEnginesStore().engines = data
  })
}
