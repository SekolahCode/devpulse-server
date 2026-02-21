import { createApp } from 'vue'
import { createPinia } from 'pinia'
import axios from 'axios'
import './style.css'
import App from './App.vue'
import router from './router/index.js'

// ── Axios auth setup ──────────────────────────────────────────────────────────
const storedToken = localStorage.getItem('devpulse_token')
if (storedToken) {
  axios.defaults.headers.common['Authorization'] = `Bearer ${storedToken}`
}

// On 401: clear stale token and redirect to login
axios.interceptors.response.use(null, (err) => {
  if (err.response?.status === 401) {
    localStorage.removeItem('devpulse_token')
    delete axios.defaults.headers.common['Authorization']
    router.push({ path: '/login', query: { redirect: router.currentRoute.value.fullPath } })
  }
  return Promise.reject(err)
})

// ── App bootstrap ─────────────────────────────────────────────────────────────
const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')
