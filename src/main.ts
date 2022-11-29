import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { router } from './router'
import { registerOauthHandlers } from './utils/oauth'

import './main.css'
import { loadSettingsFromDatabase } from './utils/settings'

const app = createApp(App)

const pinia = createPinia()

app.use(pinia)
app.use(router)
app.mount('#app')

registerOauthHandlers()

await loadSettingsFromDatabase()
