<template>
  <div class="max-w-6xl mx-auto px-6 py-8">

    <!-- Header -->
    <div class="flex items-center justify-between mb-5">
      <div class="flex items-center gap-3">
        <router-link to="/" class="text-gray-500 hover:text-gray-300 text-sm transition-colors">
          Projects
        </router-link>
        <span class="text-gray-700">/</span>
        <span class="text-[15px] font-semibold text-white">Issues</span>
        <span class="text-gray-700">·</span>
        <router-link
          :to="`/projects/${route.params.id}/releases`"
          class="text-[13px] text-gray-500 hover:text-violet-400 transition-colors flex items-center gap-1"
        >
          <svg width="11" height="11" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="8" cy="8" r="6"/><line x1="8" y1="5" x2="8" y2="8"/><line x1="8" y1="8" x2="10.5" y2="10.5"/>
          </svg>
          Releases
        </router-link>
      </div>

      <!-- Status / view tabs -->
      <div class="flex items-center bg-[#111119] border border-white/6 rounded-lg p-1 gap-0.5">
        <button
          v-for="t in TABS"
          :key="t.value"
          @click="setTab(t.value)"
          :class="tab === t.value ? 'bg-violet-600 text-white shadow' : 'text-gray-400 hover:text-gray-200'"
          class="px-3 py-1 rounded-md text-xs font-medium capitalize transition-all"
        >
          {{ t.label }}
        </button>
      </div>
    </div>

    <!-- Stats bar -->
    <div v-if="store.stats" class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-5">
      <div class="bg-[#111119] border border-white/6 rounded-xl px-4 py-3 flex flex-col gap-0.5">
        <span class="text-[10px] text-gray-500 uppercase tracking-wide font-medium">Unresolved</span>
        <span class="text-xl font-bold text-red-400">{{ store.stats.issues.unresolved }}</span>
      </div>
      <div class="bg-[#111119] border border-white/6 rounded-xl px-4 py-3 flex flex-col gap-0.5">
        <span class="text-[10px] text-gray-500 uppercase tracking-wide font-medium">New 24 h</span>
        <span class="text-xl font-bold text-amber-400">{{ store.stats.issues.new_24h }}</span>
      </div>
      <div class="bg-[#111119] border border-white/6 rounded-xl px-4 py-3 flex flex-col gap-0.5">
        <span class="text-[10px] text-gray-500 uppercase tracking-wide font-medium">Regressions</span>
        <span class="text-xl font-bold text-orange-400">{{ store.stats.issues.regressions_24h }}</span>
      </div>
      <div class="bg-[#111119] border border-white/6 rounded-xl px-4 py-3 flex flex-col gap-0.5">
        <span class="text-[10px] text-gray-500 uppercase tracking-wide font-medium">Events 24 h</span>
        <span class="text-xl font-bold text-violet-400">{{ store.stats.events_24h }}</span>
      </div>
    </div>

    <!-- Search + filters -->
    <div class="flex gap-2 mb-4">
      <input
        v-if="tab !== 'vitals'"
        v-model="search"
        @input="onSearch"
        type="text"
        placeholder="Search issues…"
        class="flex-1 bg-[#111119] border border-white/6 rounded-xl px-4 py-2.5 text-sm text-gray-200
               placeholder-gray-600 focus:outline-none focus:border-violet-500/50 transition-colors"
      />
      <div v-else class="flex-1 flex items-center gap-2 bg-[#111119] border border-white/6 rounded-xl px-4 py-2.5">
        <span class="text-xs text-violet-400 font-medium">Performance vitals</span>
        <span class="text-[10px] text-gray-600">· Web Core Vitals events</span>
      </div>
      <select
        v-model="environment"
        @change="refetch"
        class="bg-[#111119] border border-white/6 rounded-xl px-3 py-2.5 text-sm text-gray-300
               focus:outline-none focus:border-violet-500/50 transition-colors"
      >
        <option value="">All envs</option>
        <option v-for="env in ENVIRONMENTS" :key="env" :value="env" class="capitalize">{{ env }}</option>
      </select>
      <select
        v-model="release"
        @change="refetch"
        class="bg-[#111119] border border-white/6 rounded-xl px-3 py-2.5 text-sm text-gray-300
               focus:outline-none focus:border-violet-500/50 transition-colors"
      >
        <option value="">All versions</option>
        <option v-for="r in releases" :key="r.id" :value="r.version">v{{ r.version }}</option>
      </select>
    </div>

    <!-- Bulk action bar -->
    <Transition name="bulk-bar">
      <div
        v-if="selected.size > 0"
        class="flex items-center gap-3 bg-violet-600/15 border border-violet-500/30 rounded-xl px-4 py-2.5 mb-3"
      >
        <span class="text-sm text-violet-300 font-medium">{{ selected.size }} selected</span>
        <div class="flex gap-2 ml-auto">
          <button
            @click="bulkResolve"
            class="flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg bg-emerald-600/20 text-emerald-400
                   hover:bg-emerald-600/30 transition-colors font-medium"
          >
            ✓ Resolve
          </button>
          <button
            @click="bulkIgnore"
            class="flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg bg-white/5 text-gray-300
                   hover:bg-white/10 transition-colors"
          >
            Ignore
          </button>
          <button
            @click="selected = new Set()"
            class="text-xs text-gray-500 hover:text-gray-300 px-2 transition-colors"
          >
            ✕
          </button>
        </div>
      </div>
    </Transition>

    <!-- Loading skeleton -->
    <div v-if="store.loading" class="space-y-px rounded-xl overflow-hidden border border-white/6">
      <div v-for="i in 5" :key="i" class="h-13 bg-[#111119] animate-pulse" />
    </div>

    <!-- Empty state -->
    <div v-else-if="!store.issues.length" class="flex flex-col items-center justify-center py-24 text-center">
      <div class="text-3xl mb-3">{{ tab === 'vitals' ? '📊' : search ? '🔍' : '🎉' }}</div>
      <p class="text-gray-300 font-medium">
        {{ tab === 'vitals' ? 'No performance vitals recorded yet'
           : search ? 'No issues matching "' + search + '"'
           : 'No ' + tab + ' issues' }}
      </p>
      <p class="text-gray-500 text-sm mt-1">
        {{ tab === 'vitals' ? 'Vitals appear once your browser SDK reports Web Core Vitals'
           : search ? 'Try a different search term' : 'This project is clean' }}
      </p>
    </div>

    <!-- Issue table -->
    <div v-else class="rounded-xl border border-white/6 overflow-visible">

      <!-- Table header -->
      <div class="grid grid-cols-[24px_1fr_80px_80px_96px_36px] gap-0 bg-[#0d0d16] border-b border-white/5
                  text-[10px] font-semibold text-gray-500 uppercase tracking-widest">
        <div class="flex items-center justify-center py-2">
          <input
            type="checkbox"
            :checked="allSelected"
            :indeterminate="someSelected"
            @change="toggleAll"
            class="w-3 h-3 accent-violet-500 cursor-pointer"
          />
        </div>
        <span class="px-4 py-2">Issue</span>
        <span class="text-center py-2">Priority</span>
        <span class="text-right pr-3 py-2">Events</span>
        <span class="text-right pr-3 py-2">Last seen</span>
        <span></span>
      </div>

      <!-- Rows -->
      <div class="divide-y divide-white/4">
        <div
          v-for="issue in store.issues"
          :key="issue.id"
          class="grid grid-cols-[24px_1fr_80px_80px_96px_36px] gap-0 items-center bg-[#111119] hover:bg-[#13131f] transition-colors group relative"
          :class="{ 'bg-violet-600/5': selected.has(issue.id) }"
        >
          <!-- Checkbox -->
          <div class="flex items-center justify-center py-3.5">
            <input
              type="checkbox"
              :checked="selected.has(issue.id)"
              @change="toggleSelect(issue.id)"
              @click.stop
              class="w-3 h-3 accent-violet-500 cursor-pointer"
            />
          </div>

          <!-- Title + level -->
          <router-link
            :to="`/issues/${issue.id}`"
            class="flex items-center gap-2.5 px-4 py-3.5 min-w-0"
          >
            <span :class="levelDot(issue.level)" class="w-1.5 h-1.5 rounded-full shrink-0" />
            <span :class="levelBadge(issue.level)"
                  class="text-[9px] font-bold px-1.5 py-0.5 rounded uppercase tracking-wider shrink-0 hidden sm:inline">
              {{ issue.level }}
            </span>
            <p class="text-[13px] text-gray-300 truncate group-hover:text-white transition-colors leading-snug">
              {{ issue.title }}
            </p>
            <span v-if="issue.environment && issue.environment !== 'production'"
                  :class="envBadge(issue.environment)"
                  class="text-[9px] font-bold px-1.5 py-0.5 rounded uppercase tracking-wider shrink-0 hidden md:inline">
              {{ issue.environment }}
            </span>
          </router-link>

          <!-- Priority -->
          <div class="flex items-center justify-center py-3.5">
            <span v-if="issue.priority"
                  :class="priorityBadge(issue.priority)"
                  class="text-[9px] font-bold px-1.5 py-0.5 rounded uppercase tracking-wider">
              {{ issue.priority }}
            </span>
            <span v-else class="text-gray-700 text-[10px]">—</span>
          </div>

          <!-- Event count -->
          <router-link :to="`/issues/${issue.id}`"
            class="text-xs text-gray-500 tabular-nums text-right pr-3 py-3.5 shrink-0 hover:text-gray-300">
            {{ issue.event_count.toLocaleString() }}×
          </router-link>

          <!-- Last seen -->
          <router-link :to="`/issues/${issue.id}`"
            class="text-xs text-gray-500 text-right pr-3 py-3.5 shrink-0 hover:text-gray-300 tabular-nums">
            {{ timeAgo(issue.last_seen) }}
          </router-link>

          <!-- Ellipsis menu -->
          <div class="flex items-center justify-center py-3.5 shrink-0 relative" @click.prevent>
            <button
              @click.stop="toggleMenu(issue.id)"
              class="w-6 h-6 flex items-center justify-center rounded text-gray-600
                     hover:text-gray-300 hover:bg-white/8 transition-colors opacity-0 group-hover:opacity-100"
              :class="{ 'opacity-100 text-gray-300 bg-white/8': openMenu === issue.id }"
              title="Actions"
            >
              <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                <circle cx="8" cy="3" r="1.3"/><circle cx="8" cy="8" r="1.3"/><circle cx="8" cy="13" r="1.3"/>
              </svg>
            </button>

            <Transition name="dropdown">
              <div
                v-if="openMenu === issue.id"
                class="absolute right-0 top-full mt-1 w-40 bg-[#1a1a28] border border-white/10
                       rounded-lg shadow-xl shadow-black/40 z-50 overflow-hidden"
              >
                <!-- Resolve -->
                <button
                  @click.stop="action('resolve', issue.id)"
                  class="w-full flex items-center gap-2.5 px-3 py-2 text-xs transition-colors text-left"
                  :class="isPending(issue.id, 'resolve')
                    ? 'bg-emerald-500/20 text-emerald-300 font-semibold'
                    : 'text-gray-300 hover:bg-emerald-500/10 hover:text-emerald-400'"
                >
                  <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="2 8 6 12 14 4"/>
                  </svg>
                  {{ isPending(issue.id, 'resolve') ? 'Confirm resolve?' : 'Resolve' }}
                </button>
                <!-- Ignore -->
                <button
                  @click.stop="action('ignore', issue.id)"
                  class="w-full flex items-center gap-2.5 px-3 py-2 text-xs transition-colors text-left"
                  :class="isPending(issue.id, 'ignore')
                    ? 'bg-red-500/15 text-red-300 font-semibold'
                    : 'text-gray-300 hover:bg-white/5 hover:text-gray-200'"
                >
                  <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                    <circle cx="8" cy="8" r="6"/><line x1="5" y1="5" x2="11" y2="11"/>
                  </svg>
                  {{ isPending(issue.id, 'ignore') ? 'Confirm ignore?' : 'Ignore' }}
                </button>
                <!-- Cancel pending -->
                <div v-if="menuConfirm" class="border-t border-white/5">
                  <button
                    @click.stop="menuConfirm = null"
                    class="w-full px-3 py-1.5 text-[11px] text-gray-600 hover:text-gray-400 transition-colors text-left"
                  >
                    Cancel
                  </button>
                </div>
              </div>
            </Transition>
          </div>

        </div>
      </div>

      <!-- Footer -->
      <div class="px-4 py-3 bg-[#0d0d16] border-t border-white/5 flex items-center justify-between">
        <span class="text-xs text-gray-600">
          Showing <span class="text-gray-400">{{ store.issues.length }}</span> of <span class="text-gray-400">{{ store.total }}</span>
        </span>
        <button
          v-if="store.hasMore"
          @click="loadMore"
          :disabled="store.loadingMore"
          class="text-xs text-violet-400 hover:text-violet-300 disabled:opacity-50 transition-colors font-medium"
        >
          {{ store.loadingMore ? 'Loading…' : 'Load more' }}
        </button>
      </div>
    </div>

  </div>
</template>

<script setup>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import axios from 'axios'
import { useIssuesStore } from '../stores/issues'
import { useToastStore }  from '../stores/toast'
import { ENVIRONMENTS, levelBadge, levelDot, envBadge, priorityBadge } from '../composables/useColors'

const route  = useRoute()
const router = useRouter()
const store  = useIssuesStore()
const toast  = useToastStore()

const TABS = [
  { value: 'unresolved', label: 'Unresolved' },
  { value: 'resolved',   label: 'Resolved' },
  { value: 'ignored',    label: 'Ignored' },
  { value: 'vitals',     label: 'Vitals' },
]

// ── Filter state — initialised from URL query ─────────────────────────────────
const tab         = ref(route.query.tab         ?? 'unresolved')
const search      = ref(route.query.search      ?? '')
const environment = ref(route.query.env         ?? '')
const release     = ref(route.query.release     ?? '')
const releases    = ref([])
const openMenu    = ref(null)
const menuConfirm = ref(null) // { id, type } — pending confirmation for ellipsis action
const selected    = ref(new Set())

let searchTimer = null

// ── Helpers ───────────────────────────────────────────────────────────────────
function activeStatus() { return tab.value === 'vitals' ? 'unresolved' : tab.value }
function activeSearch()  { return tab.value === 'vitals' ? 'Performance vitals' : search.value }

function syncQuery() {
  router.replace({ query: {
    ...(tab.value !== 'unresolved' ? { tab: tab.value }       : {}),
    ...(search.value               ? { search: search.value } : {}),
    ...(environment.value          ? { env: environment.value } : {}),
    ...(release.value              ? { release: release.value } : {}),
  }})
}

function fetchIssues() {
  store.fetch(route.params.id, activeStatus(), {
    search:      activeSearch(),
    environment: environment.value,
    release:     release.value,
  })
}

// ── Lifecycle ─────────────────────────────────────────────────────────────────
onMounted(async () => {
  store.fetchStats()
  fetchIssues()
  document.addEventListener('click',   closeMenu)
  document.addEventListener('keydown', onKeydown)
  try {
    const { data } = await axios.get(`/api/projects/${route.params.id}/releases`)
    releases.value = data.data
  } catch {}
})

onUnmounted(() => {
  document.removeEventListener('click',   closeMenu)
  document.removeEventListener('keydown', onKeydown)
})

// ── Keyboard ──────────────────────────────────────────────────────────────────
function onKeydown(e) {
  if (e.key === 'Escape') closeMenu()
}

// ── Menu ──────────────────────────────────────────────────────────────────────
function closeMenu()      { openMenu.value = null; menuConfirm.value = null }
function toggleMenu(id)   {
  if (openMenu.value === id) { closeMenu() }
  else { openMenu.value = id; menuConfirm.value = null }
}
function isPending(id, type) {
  return menuConfirm.value?.id === id && menuConfirm.value?.type === type
}

async function action(type, id) {
  if (isPending(id, type)) {
    closeMenu()
    try {
      if (type === 'resolve') await store.resolve(id)
      else                    await store.ignore(id)
    } catch {
      toast.error(`Failed to ${type} issue`)
    }
  } else {
    menuConfirm.value = { id, type }
  }
}

// ── Tab / filter changes ──────────────────────────────────────────────────────
function setTab(t) {
  tab.value = t
  selected.value = new Set()
  syncQuery()
  fetchIssues()
}

function refetch() {
  selected.value = new Set()
  syncQuery()
  fetchIssues()
}

function onSearch() {
  clearTimeout(searchTimer)
  searchTimer = setTimeout(refetch, 300)
}

function loadMore() {
  store.fetchMore(route.params.id, activeStatus(), activeSearch(), environment.value, release.value)
}

// ── Bulk selection ────────────────────────────────────────────────────────────
const allSelected  = computed(() => store.issues.length > 0 && store.issues.every(i => selected.value.has(i.id)))
const someSelected = computed(() => !allSelected.value && store.issues.some(i => selected.value.has(i.id)))

function toggleSelect(id) {
  const s = new Set(selected.value)
  if (s.has(id)) s.delete(id)
  else s.add(id)
  selected.value = s
}

function toggleAll() {
  selected.value = allSelected.value ? new Set() : new Set(store.issues.map(i => i.id))
}

async function bulkResolve() {
  const ids = [...selected.value]
  selected.value = new Set()
  try {
    await store.bulkResolve(ids)
  } catch {
    toast.error('Failed to resolve selected issues')
  }
}

async function bulkIgnore() {
  const ids = [...selected.value]
  selected.value = new Set()
  try {
    await store.bulkIgnore(ids)
  } catch {
    toast.error('Failed to ignore selected issues')
  }
}

// ── Time formatting ───────────────────────────────────────────────────────────
function timeAgo(date) {
  const diff = Math.floor((Date.now() - new Date(date)) / 1000)
  if (diff < 60)    return `${diff}s ago`
  if (diff < 3600)  return `${Math.floor(diff / 60)}m ago`
  if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`
  return `${Math.floor(diff / 86400)}d ago`
}
</script>

<style scoped>
.dropdown-enter-active { transition: opacity 0.1s ease, transform 0.1s ease; }
.dropdown-leave-active { transition: opacity 0.08s ease, transform 0.08s ease; }
.dropdown-enter-from, .dropdown-leave-to { opacity: 0; transform: translateY(-4px) scale(0.97); }

.bulk-bar-enter-active { transition: opacity 0.15s ease, transform 0.15s ease; }
.bulk-bar-leave-active { transition: opacity 0.1s ease, transform 0.1s ease; }
.bulk-bar-enter-from, .bulk-bar-leave-to { opacity: 0; transform: translateY(-6px); }
</style>
