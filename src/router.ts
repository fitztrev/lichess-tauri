import { createRouter, createWebHistory } from 'vue-router'

import SettingsPage from './components/SettingsPage.vue'
import Dashboard from './components/Dashboard.vue'
import EngineForm from './components/EngineForm.vue'
import EnginesPage from './components/EnginesPage.vue'

const routes = [
  { path: '/', component: Dashboard },
  { path: '/engines', component: EnginesPage },
  { path: '/engines/new', component: EngineForm },
  { path: '/engines/edit/:id', component: EngineForm, name: 'editEngine' },
  { path: '/settings', component: SettingsPage },
]

export let router = createRouter({
  history: createWebHistory(),
  routes,
})
