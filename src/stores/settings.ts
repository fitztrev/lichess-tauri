import { invoke } from '@tauri-apps/api'
import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => {
    return {
      lichessHost: '',
      engineHost: '',
      providerSecret: '',

      lichess_username: '',
      lichess_token: '',
    }
  },
  getters: {
    isLoggedIn(): boolean {
      return !!this.lichess_token
    },
  },
  actions: {
    async logout() {
      await invoke('logout')
    },
  },
})
