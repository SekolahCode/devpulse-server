<template>
  <!-- Hallmark · shell: N3 side-rail · design-system: design.md · designed-as-app
       formal white rail chrome, oxblood accent, quiet left-tick active nav
       (matches app pages) over per-route workspace -->
  <div class="flex min-h-screen bg-[var(--dp-paper)] text-[var(--dp-ink)]">

    <!-- ── Desktop side rail ─────────────────────────────────────────────── -->
    <aside class="hidden lg:flex flex-col w-56 shrink-0 sticky top-0 h-screen border-r border-[var(--dp-rule)] bg-[var(--dp-surface)]">

      <!-- Brand -->
      <router-link to="/" class="flex items-center gap-2.5 px-5 h-16 shrink-0 focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]">
        <div class="w-7 h-7 rounded-md bg-[var(--dp-accent)] flex items-center justify-center text-sm text-white shrink-0">
          ⚡
        </div>
        <span class="dp-display text-[16px] text-[var(--dp-ink)] tracking-tight">DevPulse</span>
      </router-link>

      <!-- Nav -->
      <nav class="flex flex-col gap-0.5 px-3 mt-3">
        <router-link to="/"
          class="flex items-center gap-2.5 pl-3 pr-3 py-2 border-l-2 text-[13px] transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
          :class="isProjectsSection ? 'border-[var(--dp-accent)] text-[var(--dp-ink)] font-medium' : 'border-transparent text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)]'">
          <svg width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <rect x="2" y="2" width="5" height="5" rx="1"/><rect x="9" y="2" width="5" height="5" rx="1"/>
            <rect x="2" y="9" width="5" height="5" rx="1"/><rect x="9" y="9" width="5" height="5" rx="1"/>
          </svg>
          Projects
        </router-link>
        <router-link to="/dashboard"
          class="flex items-center gap-2.5 pl-3 pr-3 py-2 border-l-2 text-[13px] transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
          :class="$route.path === '/dashboard' ? 'border-[var(--dp-accent)] text-[var(--dp-ink)] font-medium' : 'border-transparent text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)]'">
          <svg width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M2 13.5V8M8 13.5V2.5M14 13.5V6"/>
          </svg>
          Analytics
        </router-link>
      </nav>

      <div class="flex-1" />

      <!-- Live status + sign out -->
      <div class="px-5 py-4 border-t border-[var(--dp-rule)] flex items-center justify-between">
        <div class="flex items-center gap-2 text-xs">
          <span
            :class="wsConnected ? 'bg-emerald-500 pulse-dot' : 'bg-red-500'"
            class="w-1.5 h-1.5 rounded-full inline-block"
          />
          <span :class="wsConnected ? 'text-emerald-700' : 'text-red-600'">
            {{ wsConnected ? 'Live' : 'Disconnected' }}
          </span>
        </div>
        <button
          v-if="isLoggedIn"
          @click="logout"
          class="text-xs text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
          title="Sign out"
        >
          Sign out
        </button>
      </div>
    </aside>

    <div class="flex-1 min-w-0 flex flex-col">

      <!-- ── Mobile top bar ──────────────────────────────────────────────── -->
      <header class="lg:hidden sticky top-0 z-40 border-b border-[var(--dp-rule)] bg-[var(--dp-surface)]/90 backdrop-blur-sm">
        <div class="px-4 h-14 flex items-center justify-between gap-3">
          <router-link to="/" class="flex items-center gap-2 shrink-0">
            <div class="w-7 h-7 rounded-md bg-[var(--dp-accent)] flex items-center justify-center text-sm text-white">⚡</div>
            <span class="dp-display text-[15px] text-[var(--dp-ink)] tracking-tight hidden min-[400px]:inline">DevPulse</span>
          </router-link>

          <nav class="flex items-center gap-1">
            <router-link to="/" class="text-xs px-2.5 py-1.5 rounded-md whitespace-nowrap transition-colors"
              :class="isProjectsSection ? 'text-[var(--dp-ink)] font-medium bg-[var(--dp-surface-2)]' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)]'">
              Projects
            </router-link>
            <router-link to="/dashboard" class="text-xs px-2.5 py-1.5 rounded-md whitespace-nowrap transition-colors"
              :class="$route.path === '/dashboard' ? 'text-[var(--dp-ink)] font-medium bg-[var(--dp-surface-2)]' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)]'">
              Analytics
            </router-link>
          </nav>

          <div class="flex items-center gap-3 shrink-0">
            <span
              :class="wsConnected ? 'bg-emerald-500 pulse-dot' : 'bg-red-500'"
              class="w-1.5 h-1.5 rounded-full inline-block"
              :title="wsConnected ? 'Live' : 'Disconnected'"
            />
            <button
              v-if="isLoggedIn"
              @click="logout"
              class="text-xs text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] transition-colors whitespace-nowrap"
              title="Sign out"
            >
              Sign out
            </button>
          </div>
        </div>
      </header>

      <!-- Page content — each route paints its own paper -->
      <main class="flex-1 min-w-0 flex flex-col">
        <router-view />
      </main>
    </div>

    <!-- Toast stack (live events + error/success notifications) -->
    <div class="fixed bottom-5 right-5 flex flex-col gap-2 z-50 pointer-events-none">
      <transition-group name="toast">

        <!-- Error / success / info toasts -->
        <div
          v-for="t in toastStore.toasts"
          :key="`t-${t.id}`"
          class="pointer-events-auto w-80 rounded-lg p-3.5 shadow-2xl flex items-start gap-3 cursor-pointer"
          :class="toastStyle(t.type)"
          @click="toastStore.dismiss(t.id)"
        >
          <span class="text-base shrink-0 mt-px">{{ toastIcon(t.type) }}</span>
          <p class="text-[13px] leading-snug flex-1">{{ t.message }}</p>
        </div>

        <!-- Live event toasts — deliberate elevated-dark chip, see design.md -->
        <div
          v-for="event in liveToasts"
          :key="event.issue_id + event.ts"
          class="pointer-events-auto w-80 rounded-lg p-3.5 shadow-2xl border"
          style="background: var(--dp-elevated-bg); border-color: var(--dp-elevated-border);"
        >
          <div class="flex items-center gap-2 mb-1.5">
            <span :class="levelBadge(event.level)" class="text-[10px] font-bold px-1.5 py-0.5 rounded-md uppercase tracking-wide">
              {{ event.level }}
            </span>
            <span class="text-[11px]" style="color: var(--dp-elevated-body);">
              {{ event.is_regression ? '↩ Regression' : '✦ New issue' }}
            </span>
          </div>
          <p class="text-[13px] truncate" style="color: var(--dp-elevated-title);">{{ event.title }}</p>
        </div>

      </transition-group>
    </div>

  </div>
</template>

<script setup>
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import axios from 'axios'
import { useIssuesStore } from './stores/issues'
import { useToastStore }  from './stores/toast'
import { levelBadge } from './composables/useColors'

const store      = useIssuesStore()
const toastStore = useToastStore()
const router     = useRouter()
const route      = useRoute()

const wsConnected = ref(false)
const liveToasts  = ref([])
const isLoggedIn  = ref(!!localStorage.getItem('devpulse_token'))

// Projects section covers the list plus everything reached from it
const isProjectsSection = computed(() =>
  route.path === '/' || route.path.startsWith('/projects') || route.path.startsWith('/issues')
)

function logout() {
  localStorage.removeItem('devpulse_token')
  delete axios.defaults.headers.common['Authorization']
  isLoggedIn.value = false
  router.push('/login')
}

const toastStyle = (type) =>
  ({ error:   'bg-red-500/15 border border-red-500/25 text-red-800',
     success:  'bg-emerald-500/15 border border-emerald-500/25 text-emerald-800',
     info:     'bg-blue-500/15 border border-blue-500/25 text-blue-800' })[type]
  ?? 'bg-[var(--dp-surface)] border border-[var(--dp-rule)] text-[var(--dp-ink-2)]'

const toastIcon = (type) =>
  ({ error: '✕', success: '✓', info: 'ℹ' })[type] ?? '•'

onMounted(() => {
  let delay = 1000

  async function connect() {
    const protocol = location.protocol === 'https:' ? 'wss' : 'ws'

    // Fetch a short-lived, single-use ticket over the authenticated REST API
    // rather than putting the long-lived admin token in the WS URL, where it
    // would land in proxy/access logs and browser history.
    let ticket = ''
    try {
      const { data } = await axios.get('/api/ws-ticket')
      ticket = data.ticket
    } catch {
      // No admin token stored, or the server has none configured (dev mode,
      // where /ws accepts unauthenticated connections) — fall back and let
      // the server decide.
    }

    const ws = new WebSocket(`${protocol}://${location.host}/ws?ticket=${encodeURIComponent(ticket)}`)

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

<style scoped>
.dp-display {
  font-family: var(--dp-font-display);
  letter-spacing: -0.01em;
}
</style>
