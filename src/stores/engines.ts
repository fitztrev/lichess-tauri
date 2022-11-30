import { invoke } from '@tauri-apps/api'
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
  defaultDepth: number
  variants: Variant[]
}

export type LichessEngine = {
  id: string
  name: string
  userId: string
  maxThreads: number
  maxHash: number
  defaultDepth: number
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
  getters: {},
  actions: {},
})
