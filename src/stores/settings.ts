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
    async logout() {
      await invoke('delete_setting', { key: 'username' })
      await invoke('delete_setting', { key: 'token' })

      this.username = ''
      this.token = ''
    },
  },
})
