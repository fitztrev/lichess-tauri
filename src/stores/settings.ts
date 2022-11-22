import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    lichessHost: 'https://lichess.org',
    externalEngineHost: 'https://engine.lichess.ovh',
    providerSecret: self.crypto.randomUUID(),
  }),
  persist: true,
})
