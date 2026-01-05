import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { createPinia } from 'pinia'
import Router from './routes'
const pinia = createPinia()

createApp(App).use(Router).use(pinia).mount('#app')
