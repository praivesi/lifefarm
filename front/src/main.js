import { createApp } from 'vue'
import App from './App.vue'
import { createVuetify } from 'vuetify'
import 'vuetify/styles'
import './assets/css/main.css'
import './assets/css/reset.css'

const vuetify = createVuetify()

createApp(App).use(vuetify).mount('#app')