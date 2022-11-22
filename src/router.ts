import { createRouter, createWebHistory } from 'vue-router'

import SettingsPage from './components/SettingsPage.vue'
import HomePage from './components/HomePage.vue'
import AddCustomEnginePage from './components/AddCustomEnginePage.vue'
import EnginesPage from './components/EnginesPage.vue'

const routes = [
  { path: '/', component: HomePage },
  { path: '/engines', component: EnginesPage },
  { path: '/engines/custom', component: AddCustomEnginePage },
  { path: '/settings', component: SettingsPage },
]

export let router = createRouter({
  history: createWebHistory(),
  routes,
})
