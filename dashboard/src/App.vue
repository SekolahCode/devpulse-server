<template>
  <!-- Hallmark · shell: floating rounded rail (matches teleradiology's DashboardSidebar)
       slate/maroon chrome, collapsible, active-tab = text color only (no fill),
       lucide nav icons — over per-route workspace -->
  <div class="min-h-screen bg-[var(--dp-paper)]">

    <!-- ── Desktop floating sidebar ──────────────────────────────────────── -->
    <aside
      class="fixed inset-y-4 left-4 z-30 hidden lg:flex flex-col rounded-2xl border border-[var(--dp-rule)] bg-[var(--dp-paper)]/95 shadow-xl shadow-black/5 ring-1 ring-black/5 backdrop-blur transition-[width] duration-300 ease-out"
      :class="sidebarOpen ? 'w-56' : 'w-16'"
    >
      <!-- Collapse / expand toggle -->
      <button
        type="button"
        @click="sidebarOpen = !sidebarOpen"
        class="absolute -right-3 top-6 z-40 flex h-6 w-6 items-center justify-center rounded-full bg-[var(--dp-surface)] text-[var(--dp-accent)] shadow-md ring-1 ring-black/5 transition-transform hover:scale-105 focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
        :aria-label="sidebarOpen ? 'Collapse sidebar' : 'Expand sidebar'"
      >
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
          class="transition-transform duration-300" :class="sidebarOpen ? '' : 'rotate-180'">
          <path d="M10 3L5 8l5 5"/>
        </svg>
      </button>

      <!-- Brand -->
      <router-link to="/" class="flex items-center pt-5 pb-4 focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
        :class="sidebarOpen ? 'gap-5 px-5' : 'justify-center gap-0 px-0'">
        <div class="w-7 h-7 rounded-lg bg-[var(--dp-accent)] flex items-center justify-center text-sm text-white shrink-0">
          ⚡
        </div>
        <div class="min-w-0 overflow-hidden leading-tight transition-[max-width,opacity] duration-200 ease-out"
          :class="sidebarOpen ? 'max-w-[10rem] opacity-100 delay-150' : 'max-w-0 opacity-0'">
          <span class="block truncate text-sm font-semibold text-[var(--dp-ink)] tracking-tight">DevPulse</span>
        </div>
      </router-link>

      <!-- Nav -->
      <nav class="flex-1 overflow-y-auto pb-4" :class="sidebarOpen ? 'space-y-1 px-3' : 'space-y-2 px-1.5'">
        <router-link to="/"
          class="group relative flex items-center rounded-xl py-2.5 text-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
          :class="[sidebarOpen ? 'justify-start gap-5 px-3' : 'justify-center gap-0 px-0',
                   $route.path === '/' ? 'text-[var(--dp-accent)]' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-accent)]']">
          <LayoutDashboard :size="18" class="shrink-0" />
          <span class="min-w-0 flex-1 truncate text-left transition-[max-width,opacity] duration-200 ease-out"
            :class="sidebarOpen ? 'max-w-[10rem] opacity-100 delay-150' : 'max-w-0 opacity-0'">Dashboard</span>
        </router-link>
        <router-link to="/projects"
          class="group relative flex items-center rounded-xl py-2.5 text-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
          :class="[sidebarOpen ? 'justify-start gap-5 px-3' : 'justify-center gap-0 px-0',
                   isProjectsSection ? 'text-[var(--dp-accent)]' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-accent)]']">
          <FolderKanban :size="18" class="shrink-0" />
          <span class="min-w-0 flex-1 truncate text-left transition-[max-width,opacity] duration-200 ease-out"
            :class="sidebarOpen ? 'max-w-[10rem] opacity-100 delay-150' : 'max-w-0 opacity-0'">Projects</span>
        </router-link>
        <router-link to="/analytics"
          class="group relative flex items-center rounded-xl py-2.5 text-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
          :class="[sidebarOpen ? 'justify-start gap-5 px-3' : 'justify-center gap-0 px-0',
                   $route.path === '/analytics' ? 'text-[var(--dp-accent)]' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-accent)]']">
          <BarChart3 :size="18" class="shrink-0" />
          <span class="min-w-0 flex-1 truncate text-left transition-[max-width,opacity] duration-200 ease-out"
            :class="sidebarOpen ? 'max-w-[10rem] opacity-100 delay-150' : 'max-w-0 opacity-0'">Analytics</span>
        </router-link>
      </nav>

      <!-- Live status + sign out -->
      <div class="border-t border-[var(--dp-rule)] p-4">
        <div class="flex items-center" :class="sidebarOpen ? 'justify-between gap-3' : 'flex-col gap-2'">
          <div class="flex items-center gap-2 text-xs" :title="wsConnected ? 'Live' : 'Disconnected'">
            <span
              :class="wsConnected ? 'bg-emerald-500 pulse-dot' : 'bg-red-500'"
              class="w-1.5 h-1.5 rounded-full inline-block shrink-0"
            />
            <span v-if="sidebarOpen" :class="wsConnected ? 'text-emerald-700' : 'text-red-600'">
              {{ wsConnected ? 'Live' : 'Disconnected' }}
            </span>
          </div>
          <button
            v-if="isLoggedIn"
            @click="logout"
            class="text-xs text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
            title="Sign out"
          >
            {{ sidebarOpen ? 'Sign out' : '⏻' }}
          </button>
        </div>
      </div>
    </aside>

    <div class="flex min-h-screen flex-col transition-[margin] duration-300 ease-out" :class="sidebarOpen ? 'lg:ml-64' : 'lg:ml-24'">

      <!-- ── Mobile top bar ──────────────────────────────────────────────── -->
      <header class="lg:hidden sticky top-0 z-40 border-b border-[var(--dp-rule)] bg-[var(--dp-paper)]/90 backdrop-blur-sm">
        <div class="px-4 h-14 flex items-center justify-between gap-3">
          <router-link to="/" class="flex items-center gap-2 shrink-0">
            <div class="w-7 h-7 rounded-lg bg-[var(--dp-accent)] flex items-center justify-center text-sm text-white">⚡</div>
            <span class="text-[15px] font-semibold text-[var(--dp-ink)] tracking-tight hidden min-[400px]:inline">DevPulse</span>
          </router-link>

          <nav class="flex items-center gap-1">
            <router-link to="/" class="text-xs px-2.5 py-1.5 rounded-lg whitespace-nowrap transition-colors"
              :class="$route.path === '/' ? 'text-[var(--dp-accent)] font-medium' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-accent)]'">
              Dashboard
            </router-link>
            <router-link to="/projects" class="text-xs px-2.5 py-1.5 rounded-lg whitespace-nowrap transition-colors"
              :class="isProjectsSection ? 'text-[var(--dp-accent)] font-medium' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-accent)]'">
              Projects
            </router-link>
            <router-link to="/analytics" class="text-xs px-2.5 py-1.5 rounded-lg whitespace-nowrap transition-colors"
              :class="$route.path === '/analytics' ? 'text-[var(--dp-accent)] font-medium' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-accent)]'">
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
          class="pointer-events-auto w-80 rounded-xl p-3.5 shadow-2xl flex items-start gap-3 cursor-pointer"
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
          class="pointer-events-auto w-80 rounded-xl p-3.5 shadow-2xl border"
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
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { LayoutDashboard, FolderKanban, BarChart3 } from 'lucide-vue-next'
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
const sidebarOpen = ref(true)

function checkScreenSize() {
  sidebarOpen.value = window.innerWidth >= 1024
}

// Projects section covers the list plus everything reached from it
const isProjectsSection = computed(() =>
  route.path.startsWith('/projects') || route.path.startsWith('/issues')
)

function logout() {
  closeWs()
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

// ── Live WebSocket connection ─────────────────────────────────────────────────
let ws = null
let manualClose = false
let reconnectDelay = 1000

async function connectWs() {
  const protocol = location.protocol === 'https:' ? 'wss' : 'ws'

  // The deployed ws_handler currently checks ?token=<ADMIN_TOKEN> directly
  // (a ticket-issuing /api/ws-ticket endpoint was never actually shipped to
  // this route table — calling it 404s to the SPA fallback, which handed
  // back index.html and produced a literal "?ticket=undefined" query param).
  // Send the same token already used for the Authorization header instead;
  // it's empty in dev mode, where the server accepts unauthenticated conns.
  const token = localStorage.getItem('devpulse_token') ?? ''

  ws = new WebSocket(`${protocol}://${location.host}/ws?token=${encodeURIComponent(token)}`)

  ws.onopen = () => {
    wsConnected.value = true
    reconnectDelay = 1000
  }

  ws.onclose = () => {
    wsConnected.value = false
    if (manualClose) { manualClose = false; return }
    // Don't chase a 401 in a retry loop once the token's gone — the
    // 'devpulse:login' listener below reconnects once one exists again.
    if (!localStorage.getItem('devpulse_token')) return
    setTimeout(connectWs, reconnectDelay)
    reconnectDelay = Math.min(reconnectDelay * 2, 30_000)
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

function closeWs() {
  if (ws && (ws.readyState === WebSocket.CONNECTING || ws.readyState === WebSocket.OPEN)) {
    manualClose = true
    ws.close()
  }
}

onMounted(() => {
  checkScreenSize()
  window.addEventListener('resize', checkScreenSize)

  // Only attempt the handshake once credentials actually exist — otherwise
  // this always 401s once on a cold, unauthenticated load before the
  // reconnect backoff quietly recovers it a second later.
  if (localStorage.getItem('devpulse_token')) connectWs()

  // Fired by LoginView right after a token is saved, so a fresh login
  // connects immediately instead of waiting on the reconnect backoff.
  window.addEventListener('devpulse:login', connectWs)
})

onUnmounted(() => {
  window.removeEventListener('resize', checkScreenSize)
  window.removeEventListener('devpulse:login', connectWs)
})
</script>
