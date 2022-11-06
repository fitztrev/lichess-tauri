import { defineStore } from "pinia";

export const useSettingsStore = defineStore("settings", {
  state: () => ({
    lichessHost:
      "https://8080-lichessorg-lilagitpod-15zotrnb8h8.ws-us73.gitpod.io",
    externalEngineHost:
      "https://9666-lichessorg-lilagitpod-15zotrnb8h8.ws-us73.gitpod.io",
  }),
  persist: true,
});
