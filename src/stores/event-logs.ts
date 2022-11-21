import { defineStore } from 'pinia'

export interface LichessWorkEvent {
  id: number
  event: string
  windowLabel: string
  payload: {
    event: string
    message: string
    analysis_request: false
  }
}

export const useEventLogsStore = defineStore('event-logs', {
  state: () => ({
    logs: [] as LichessWorkEvent[],
  }),
  actions: {
    add(event: LichessWorkEvent) {
      this.logs.push(event)
    },
  },
})
