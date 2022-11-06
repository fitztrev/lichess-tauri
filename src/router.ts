import {createRouter, createWebHistory} from "vue-router";

import SettingsPage from "./components/SettingsPage.vue";
import HomePage from "./components/HomePage.vue";
import AddEnginePage from "./components/AddEnginePage.vue";
import EnginesPage from "./components/EnginesPage.vue";

const routes = [
  { path: "/", component: HomePage },
  { path: "/engines", component: EnginesPage },
  { path: "/engines/new", component: AddEnginePage },
  { path: "/settings", component: SettingsPage },
]

export default createRouter({
    history: createWebHistory(),
    routes,
})
