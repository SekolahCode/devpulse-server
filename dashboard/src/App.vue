<template>
  <div class="flex flex-col min-h-screen bg-[#0b0b12] text-[#e8e8f0]">

    <!-- Navbar -->
    <header class="sticky top-0 z-40 border-b border-white/5 bg-[#0b0b12]/90 backdrop-blur-sm">
      <div class="max-w-6xl mx-auto px-6 h-14 flex items-center justify-between">

        <!-- Logo -->
        <div class="flex items-center gap-5">
          <router-link to="/" class="flex items-center gap-2.5">
            <div class="w-7 h-7 rounded-lg bg-violet-600 flex items-center justify-center text-sm font-bold shadow-lg shadow-violet-900/40">
              ⚡
            </div>
            <span class="font-semibold text-[15px] text-white tracking-tight">DevPulse</span>
          </router-link>

          <!-- Nav links -->
          <nav class="hidden sm:flex items-center gap-1">
            <router-link to="/" class="text-xs px-2.5 py-1.5 rounded-md text-gray-400 hover:text-white hover:bg-white/5 transition-colors"
              :class="{ 'text-white bg-white/8': $route.path === '/' }">
              Projects
            </router-link>
            <router-link to="/dashboard" class="text-xs px-2.5 py-1.5 rounded-md text-gray-400 hover:text-white hover:bg-white/5 transition-colors"
              :class="{ 'text-white bg-white/8': $route.path === '/dashboard' }">
              Analytics
            </router-link>
          </nav>
        </div>

        <div class="flex items-center gap-4">
          <!-- Live status -->
          <div class="flex items-center gap-2 text-xs">
            <span
              :class="wsConnected ? 'bg-emerald-500 pulse-dot' : 'bg-red-500'"
              class="w-1.5 h-1.5 rounded-full inline-block"
            />
            <span :class="wsConnected ? 'text-emerald-400' : 'text-red-400'">
              {{ wsConnected ? 'Live' : 'Disconnected' }}
            </span>
          </div>

          <!-- Logout -->
          <button
            v-if="isLoggedIn"
            @click="logout"
            class="text-xs text-gray-500 hover:text-gray-300 transition-colors"
            title="Sign out"
          >
            Sign out
          </button>
        </div>
      </div>
    </header>

    <!-- Page content -->
    <main class="flex-1">
      <router-view />
    </main>

    <!-- Toast stack (live events + error/success notifications) -->
    <div class="fixed bottom-5 right-5 flex flex-col gap-2 z-50 pointer-events-none">
      <transition-group name="toast">

        <!-- Error / success / info toasts -->
        <div
          v-for="t in toastStore.toasts"
          :key="`t-${t.id}`"
          class="pointer-events-auto w-80 rounded-xl p-3.5 shadow-2xl flex items-start gap-3 cursor-pointer"
          :class="toastStyle(t.type)"
          @click="toastStore.dismiss(t.id)"
        >
          <span class="text-base shrink-0 mt-px">{{ toastIcon(t.type) }}</span>
          <p class="text-[13px] leading-snug flex-1">{{ t.message }}</p>
        </div>

        <!-- Live event toasts -->
        <div
          v-for="event in liveToasts"
          :key="event.issue_id + event.ts"
          class="pointer-events-auto w-80 bg-[#17171f] border border-white/8 rounded-xl p-3.5 shadow-2xl"
        >
          <div class="flex items-center gap-2 mb-1.5">
            <span :class="levelBadge(event.level)" class="text-[10px] font-bold px-1.5 py-0.5 rounded-md uppercase tracking-wide">
              {{ event.level }}
            </span>
            <span class="text-[11px] text-gray-400">
              {{ event.is_regression ? '↩ Regression' : '✦ New issue' }}
            </span>
          </div>
          <p class="text-[13px] text-gray-200 truncate">{{ event.title }}</p>
        </div>

      </transition-group>
    </div>

  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'
import { useIssuesStore } from './stores/issues'
import { useToastStore }  from './stores/toast'
import { levelBadge } from './composables/useColors'

const store      = useIssuesStore()
const toastStore = useToastStore()
const router     = useRouter()

const wsConnected = ref(false)
const liveToasts  = ref([])
const isLoggedIn  = ref(!!localStorage.getItem('devpulse_token'))

function logout() {
  localStorage.removeItem('devpulse_token')
  delete axios.defaults.headers.common['Authorization']
  isLoggedIn.value = false
  router.push('/login')
}

const toastStyle = (type) =>
  ({ error:   'bg-red-500/15 border border-red-500/25 text-red-200',
     success:  'bg-emerald-500/15 border border-emerald-500/25 text-emerald-200',
     info:     'bg-blue-500/15 border border-blue-500/25 text-blue-200' })[type]
  ?? 'bg-[#17171f] border border-white/8 text-gray-200'

const toastIcon = (type) =>
  ({ error: '✕', success: '✓', info: 'ℹ' })[type] ?? '•'

onMounted(() => {
  let delay = 1000

  function connect() {
    const protocol = location.protocol === 'https:' ? 'wss' : 'ws'
    const token = localStorage.getItem('devpulse_token') ?? ''
    const ws = new WebSocket(`${protocol}://${location.host}/ws?token=${encodeURIComponent(token)}`)

    ws.onopen = () => {
      wsConnected.value = true
      delay = 1000
    }

    ws.onclose = () => {
      wsConnected.value = false
      setTimeout(connect, delay)
      delay = Math.min(delay * 2, 30_000)
    }

    ws.onmessage = ({ data }) => {
      let event
      try { event = JSON.parse(data) } catch { return }
      if (event.type !== 'new_event') return
      store.addLiveEvent(event)

      const toast = { ...event, ts: Date.now() }
      liveToasts.value.unshift(toast)
      setTimeout(() => {
        const idx = liveToasts.value.indexOf(toast)
        if (idx !== -1) liveToasts.value.splice(idx, 1)
      }, 5000)
    }
  }

  connect()
})
</script>
