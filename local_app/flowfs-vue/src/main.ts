import { createApp } from "vue"

import VueClickAway from "vue3-click-away"

import App from "./App.vue"
import router from "./router"

const app = createApp(App)

app.use(VueClickAway)

app.use(router).mount("#app")
