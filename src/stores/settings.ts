import { invoke } from '@tauri-apps/api'
import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', {
  state: () => {
    return {
      lichessHost: '',
      engineHost: '',
      providerSecret: '',

      username: '',
      token: '',
    }
  },
  getters: {
    isLoggedIn(): boolean {
      return !!this.token
    },
  },
  actions: {
    logout() {
      this.username = ''
      this.token = ''
    },
  },
})
