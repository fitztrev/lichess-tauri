import { defineStore } from "pinia";

export const useUserStore = defineStore('user', {
    state: () => ({
        username: '',
        token: '',
    }),
    actions: {
        destroy() {
            this.username = ''
            this.token = ''
        },
    },
    persist: true,
});
