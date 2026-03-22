<template>
  <div class="flex flex-col min-h-screen bg-[#0b0b12] text-[#e8e8f0]">

    <!-- Navbar -->
    <header class="sticky top-0 z-40 border-b border-white/5 bg-[#0b0b12]/90 backdrop-blur-sm">
      <div class="max-w-6xl mx-auto px-6 h-14 flex items-center justify-between">

        <!-- Logo -->
        <router-link to="/" class="flex items-center gap-2.5">
          <div class="w-7 h-7 rounded-lg bg-violet-600 flex items-center justify-center text-sm font-bold shadow-lg shadow-violet-900/40">
            ⚡
          </div>
          <span class="font-semibold text-[15px] text-white tracking-tight">DevPulse</span>
        </router-link>

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

    <!-- Toast stack -->
    <div class="fixed bottom-5 right-5 flex flex-col gap-2 z-50 pointer-events-none">
      <transition-group name="toast">
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

const store       = useIssuesStore()
const router      = useRouter()
const wsConnected = ref(false)
const liveToasts  = ref([])
const isLoggedIn  = ref(!!localStorage.getItem('devpulse_token'))

function logout() {
  localStorage.removeItem('devpulse_token')
  delete axios.defaults.headers.common['Authorization']
  isLoggedIn.value = false
  router.push('/login')
}

const levelBadge = (level) =>
  ({ error: 'bg-red-500/15 text-red-400', warning: 'bg-amber-500/15 text-amber-400', info: 'bg-blue-500/15 text-blue-400' })[level]
  ?? 'bg-gray-500/15 text-gray-400'

onMounted(() => {
  let delay = 1000

  function connect() {
    const protocol = location.protocol === 'https:' ? 'wss' : 'ws'
    const ws = new WebSocket(`${protocol}://${location.host}/ws`)

    ws.onopen = () => {
      wsConnected.value = true
      delay = 1000  // reset backoff on successful connect
    }

    ws.onclose = () => {
      wsConnected.value = false
      setTimeout(connect, delay)
      delay = Math.min(delay * 2, 30_000)  // exponential backoff, max 30s
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
