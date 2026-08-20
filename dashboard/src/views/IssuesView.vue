<template>
  <div class="issues-root flex-1 w-full">
    <div class="max-w-6xl mx-auto px-6 py-8">

      <!-- Header -->
      <div class="flex flex-wrap items-center justify-between gap-3 mb-5">
        <div class="flex items-center gap-3">
          <router-link to="/" class="text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] text-sm transition-colors">
            Projects
          </router-link>
          <span class="text-[var(--dp-rule)]">/</span>
          <h1 class="dp-display text-[17px] text-[var(--dp-ink)]">Issues</h1>
          <span class="text-[var(--dp-rule)]">·</span>
          <router-link
            :to="`/projects/${route.params.id}/releases`"
            class="text-[13px] text-[var(--dp-ink-3)] hover:text-[var(--dp-accent)] transition-colors flex items-center gap-1"
          >
            <svg width="11" height="11" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="8" cy="8" r="6"/><line x1="8" y1="5" x2="8" y2="8"/><line x1="8" y1="8" x2="10.5" y2="10.5"/>
            </svg>
            Releases
          </router-link>
        </div>

        <!-- Status / view tabs -->
        <div class="flex items-center bg-[var(--dp-surface-2)] border border-[var(--dp-rule)] rounded-lg p-1 gap-0.5">
          <button
            v-for="t in TABS"
            :key="t.value"
            @click="setTab(t.value)"
            :class="tab === t.value ? 'bg-[var(--dp-accent)] text-white' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)]'"
            class="px-3 py-1 rounded-md text-xs font-medium capitalize transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
          >
            {{ t.label }}
          </button>
        </div>
      </div>

      <!-- Stats strip — quiet label:value line, no boxed tiles -->
      <div v-if="store.stats" class="flex flex-wrap items-center gap-x-8 gap-y-2 py-3 mb-5 border-y border-[var(--dp-rule)]">
        <div class="flex items-baseline gap-2">
          <span class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium">Unresolved</span>
          <span class="dp-mono tabular-nums text-base font-medium text-[var(--dp-danger)]">{{ store.stats.issues.unresolved }}</span>
        </div>
        <div class="flex items-baseline gap-2">
          <span class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium">New 24 h</span>
          <span class="dp-mono tabular-nums text-base font-medium text-amber-700">{{ store.stats.issues.new_24h }}</span>
        </div>
        <div class="flex items-baseline gap-2">
          <span class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium">Regressions</span>
          <span class="dp-mono tabular-nums text-base font-medium text-orange-700">{{ store.stats.issues.regressions_24h }}</span>
        </div>
        <div class="flex items-baseline gap-2">
          <span class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium">Events 24 h</span>
          <span class="dp-mono tabular-nums text-base font-medium text-[var(--dp-accent)]">{{ store.stats.events_24h }}</span>
        </div>
      </div>

      <!-- Search + filters -->
      <div class="flex flex-wrap gap-2 mb-4">
        <input
          v-if="tab !== 'vitals'"
          v-model="search"
          @input="onSearch"
          type="text"
          placeholder="Search issues…"
          class="flex-1 min-w-[10rem] bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-md px-4 py-2.5 text-sm text-[var(--dp-ink)]
                 placeholder-[var(--dp-ink-3)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors"
        />
        <div v-else class="flex-1 min-w-[10rem] flex items-center gap-2 bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-md px-4 py-2.5">
          <span class="text-xs text-[var(--dp-accent)] font-medium">Performance vitals</span>
          <span class="text-[10px] text-[var(--dp-ink-3)]">· Web Core Vitals events</span>
        </div>
        <select
          v-model="environment"
          @change="refetch"
          class="flex-1 sm:flex-none min-w-[7rem] bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5 text-sm text-[var(--dp-ink-2)]
                 focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors"
        >
          <option value="">All envs</option>
          <option v-for="env in ENVIRONMENTS" :key="env" :value="env" class="capitalize">{{ env }}</option>
        </select>
        <select
          v-model="release"
          @change="refetch"
          class="flex-1 sm:flex-none min-w-[7rem] bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5 text-sm text-[var(--dp-ink-2)]
                 focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors"
        >
          <option value="">All versions</option>
          <option v-for="r in releases" :key="r.id" :value="r.version">v{{ r.version }}</option>
        </select>
      </div>

      <!-- Bulk action bar -->
      <Transition name="bulk-bar">
        <div
          v-if="selected.size > 0"
          class="flex items-center gap-3 bg-[var(--dp-accent-soft)] border border-[var(--dp-accent)]/25 rounded-md px-4 py-2.5 mb-3"
        >
          <span class="text-sm text-[var(--dp-accent)] font-medium">{{ selected.size }} selected</span>
          <div class="flex gap-2 ml-auto">
            <button
              @click="bulkResolve"
              class="flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-md bg-emerald-500/10 text-emerald-700
                     hover:bg-emerald-500/15 transition-colors font-medium"
            >
              ✓ Resolve
            </button>
            <button
              @click="bulkIgnore"
              class="flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-md bg-[var(--dp-surface-2)] text-[var(--dp-ink-2)]
                     hover:bg-[var(--dp-rule)] transition-colors"
            >
              Ignore
            </button>
            <button
              @click="selected = new Set()"
              class="text-xs text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] px-2 transition-colors"
            >
              ✕
            </button>
          </div>
        </div>
      </Transition>

      <!-- Loading skeleton -->
      <div v-if="store.loading" class="space-y-px rounded-lg overflow-hidden border border-[var(--dp-rule)]">
        <div v-for="i in 5" :key="i" class="h-13 bg-[var(--dp-surface-2)] animate-pulse" />
      </div>

      <!-- Empty state -->
      <div v-else-if="!store.issues.length" class="flex flex-col items-center justify-center py-24 text-center">
        <div class="text-3xl mb-3">{{ tab === 'vitals' ? '📊' : search ? '🔍' : '🎉' }}</div>
        <p class="text-[var(--dp-ink)] font-medium">
          {{ tab === 'vitals' ? 'No performance vitals recorded yet'
             : search ? 'No issues matching "' + search + '"'
             : 'No ' + tab + ' issues' }}
        </p>
        <p class="text-[var(--dp-ink-2)] text-sm mt-1">
          {{ tab === 'vitals' ? 'Vitals appear once your browser SDK reports Web Core Vitals'
             : search ? 'Try a different search term' : 'This project is clean' }}
        </p>
      </div>

      <!-- Ledger table — hairline top rule, no boxed panel -->
      <div v-else class="border-t border-[var(--dp-rule)] overflow-visible">

        <!-- Table header — 3 cols on mobile (checkbox/issue/menu), full 6 from md -->
        <div class="grid grid-cols-[24px_1fr_36px] md:grid-cols-[24px_1fr_80px_80px_96px_36px] gap-0 border-b border-[var(--dp-rule)]
                    text-[10px] font-semibold text-[var(--dp-ink-3)] uppercase tracking-widest">
          <div class="flex items-center justify-center py-2">
            <input
              type="checkbox"
              :checked="allSelected"
              :indeterminate="someSelected"
              @change="toggleAll"
              class="w-3 h-3 accent-[var(--dp-accent)] cursor-pointer"
            />
          </div>
          <span class="px-4 py-2">Issue</span>
          <span class="hidden md:block text-center py-2">Priority</span>
          <span class="hidden md:block text-right pr-3 py-2">Events</span>
          <span class="hidden md:block text-right pr-3 py-2">Last seen</span>
          <span></span>
        </div>

        <!-- Rows -->
        <div class="divide-y divide-[var(--dp-rule)]">
          <div
            v-for="issue in store.issues"
            :key="issue.id"
            class="grid grid-cols-[24px_1fr_36px] md:grid-cols-[24px_1fr_80px_80px_96px_36px] gap-0 items-center hover:bg-[var(--dp-surface-2)]/60 transition-colors group relative"
            :class="selected.has(issue.id) ? 'bg-[var(--dp-accent-soft)]/50' : 'bg-[var(--dp-surface)]'"
          >
            <!-- Checkbox -->
            <div class="flex items-center justify-center py-3.5">
              <input
                type="checkbox"
                :checked="selected.has(issue.id)"
                @change="toggleSelect(issue.id)"
                @click.stop
                class="w-3 h-3 accent-[var(--dp-accent)] cursor-pointer"
              />
            </div>

            <!-- Title + level (+ mobile-only meta row) -->
            <router-link
              :to="`/issues/${issue.id}`"
              class="flex flex-col md:flex-row md:items-center gap-1 md:gap-2.5 px-4 py-3.5 min-w-0"
            >
              <div class="flex items-center gap-2.5 min-w-0">
                <span :class="levelDot(issue.level)" class="w-1.5 h-1.5 rounded-full shrink-0" />
                <span :class="levelBadge(issue.level)"
                      class="text-[9px] font-bold px-1.5 py-0.5 rounded uppercase tracking-wider shrink-0 hidden sm:inline">
                  {{ issue.level }}
                </span>
                <p class="text-[13px] text-[var(--dp-ink)] truncate group-hover:text-[var(--dp-accent)] transition-colors leading-snug">
                  {{ issue.title }}
                </p>
                <span v-if="issue.environment && issue.environment !== 'production'"
                      :class="envBadge(issue.environment)"
                      class="text-[9px] font-bold px-1.5 py-0.5 rounded uppercase tracking-wider shrink-0 hidden md:inline">
                  {{ issue.environment }}
                </span>
              </div>
              <!-- Mobile-only: priority / events / last seen, collapsed under the title -->
              <div class="flex md:hidden items-center gap-2.5 pl-4">
                <span v-if="issue.priority"
                      :class="priorityBadge(issue.priority)"
                      class="text-[9px] font-bold px-1.5 py-0.5 rounded uppercase tracking-wider shrink-0">
                  {{ issue.priority }}
                </span>
                <span class="dp-mono text-[11px] text-[var(--dp-ink-3)] tabular-nums shrink-0">{{ issue.event_count.toLocaleString() }}×</span>
                <span class="dp-mono text-[11px] text-[var(--dp-ink-3)] tabular-nums shrink-0">{{ timeAgo(issue.last_seen) }}</span>
              </div>
            </router-link>

            <!-- Priority (desktop only) -->
            <div class="hidden md:flex items-center justify-center py-3.5">
              <span v-if="issue.priority"
                    :class="priorityBadge(issue.priority)"
                    class="text-[9px] font-bold px-1.5 py-0.5 rounded uppercase tracking-wider">
                {{ issue.priority }}
              </span>
              <span v-else class="text-[var(--dp-ink-3)] text-[10px]">—</span>
            </div>

            <!-- Event count (desktop only) -->
            <router-link :to="`/issues/${issue.id}`"
              class="hidden md:block dp-mono text-xs text-[var(--dp-ink-2)] tabular-nums text-right pr-3 py-3.5 shrink-0 hover:text-[var(--dp-ink)]">
              {{ issue.event_count.toLocaleString() }}×
            </router-link>

            <!-- Last seen (desktop only) -->
            <router-link :to="`/issues/${issue.id}`"
              class="hidden md:block dp-mono text-xs text-[var(--dp-ink-2)] text-right pr-3 py-3.5 shrink-0 hover:text-[var(--dp-ink)] tabular-nums">
              {{ timeAgo(issue.last_seen) }}
            </router-link>

            <!-- Ellipsis menu -->
            <div class="flex items-center justify-center py-3.5 shrink-0 relative" @click.prevent>
              <button
                @click.stop="toggleMenu(issue.id)"
                class="w-6 h-6 flex items-center justify-center rounded text-[var(--dp-ink-3)]
                       hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)] transition-colors opacity-0 group-hover:opacity-100 focus-visible:opacity-100 focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
                :class="{ 'opacity-100 text-[var(--dp-ink)] bg-[var(--dp-surface-2)]': openMenu === issue.id }"
                title="Actions"
              >
                <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                  <circle cx="8" cy="3" r="1.3"/><circle cx="8" cy="8" r="1.3"/><circle cx="8" cy="13" r="1.3"/>
                </svg>
              </button>

              <Transition name="dropdown">
                <div
                  v-if="openMenu === issue.id"
                  class="dv-dropdown absolute right-0 top-full mt-1 w-40 bg-[var(--dp-surface)] border border-[var(--dp-rule)]
                         rounded-lg z-50 overflow-hidden"
                >
                  <!-- Resolve -->
                  <button
                    @click.stop="action('resolve', issue.id)"
                    class="w-full flex items-center gap-2.5 px-3 py-2 text-xs transition-colors text-left"
                    :class="isPending(issue.id, 'resolve')
                      ? 'bg-emerald-500/15 text-emerald-700 font-semibold'
                      : 'text-[var(--dp-ink-2)] hover:bg-emerald-500/10 hover:text-emerald-700'"
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
                      ? 'bg-red-500/10 text-red-700 font-semibold'
                      : 'text-[var(--dp-ink-2)] hover:bg-[var(--dp-surface-2)] hover:text-[var(--dp-ink)]'"
                  >
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                      <circle cx="8" cy="8" r="6"/><line x1="5" y1="5" x2="11" y2="11"/>
                    </svg>
                    {{ isPending(issue.id, 'ignore') ? 'Confirm ignore?' : 'Ignore' }}
                  </button>
                  <!-- Cancel pending -->
                  <div v-if="menuConfirm" class="border-t border-[var(--dp-rule)]">
                    <button
                      @click.stop="menuConfirm = null"
                      class="w-full px-3 py-1.5 text-[11px] text-[var(--dp-ink-3)] hover:text-[var(--dp-ink-2)] transition-colors text-left"
                    >
                      Cancel
                    </button>
                  </div>
                </div>
              </Transition>
            </div>

          </div>
        </div>

        <!-- Pagination footer -->
        <div class="px-1 py-3 border-t border-[var(--dp-rule)] flex items-center justify-between gap-4">

          <!-- Row count -->
          <span class="dp-mono text-xs text-[var(--dp-ink-3)] tabular-nums shrink-0">
            <span class="text-[var(--dp-ink-2)]">{{ pageStart }}–{{ pageEnd }}</span>
            of
            <span class="text-[var(--dp-ink-2)]">{{ store.total }}</span>
          </span>

          <!-- Page buttons -->
          <div class="flex items-center gap-1">
            <!-- Prev -->
            <button
              @click="goToPage(store.page - 1)"
              :disabled="store.page <= 1"
              class="w-7 h-7 flex items-center justify-center rounded-md text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-rule)]
                     disabled:opacity-30 disabled:cursor-not-allowed transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
              aria-label="Previous page"
            >
              <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="10 4 6 8 10 12"/>
              </svg>
            </button>

            <!-- Page numbers -->
            <span
              v-for="(p, i) in pageButtons"
              :key="i"
            >
              <span v-if="p === '...'" class="w-7 h-7 flex items-center justify-center text-xs text-[var(--dp-ink-3)]">…</span>
              <button
                v-else
                @click="goToPage(p)"
                :class="p === store.page
                  ? 'bg-[var(--dp-accent)] text-white'
                  : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-rule)]'"
                class="dp-mono w-7 h-7 flex items-center justify-center rounded-md text-xs font-medium transition-colors tabular-nums focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
              >
                {{ p }}
              </button>
            </span>

            <!-- Next -->
            <button
              @click="goToPage(store.page + 1)"
              :disabled="store.page >= store.totalPages"
              class="w-7 h-7 flex items-center justify-center rounded-md text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-rule)]
                     disabled:opacity-30 disabled:cursor-not-allowed transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
              aria-label="Next page"
            >
              <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="6 4 10 8 6 12"/>
              </svg>
            </button>
          </div>

          <!-- Per-page label -->
          <span class="text-xs text-[var(--dp-ink-3)] shrink-0 hidden sm:block">
            {{ store.perPage }} / page
          </span>

        </div>
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
import { ENVIRONMENTS } from '../composables/useColors'

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

// ── Light-surface badge colors ────────────────────────────────────────────────
// Page-scoped (not the shared useColors.js helpers) — those are tuned for dark
// backgrounds and still serve App.vue's toast + the unmigrated IssueDetails.vue.
const levelBadge = (level) =>
  ({ error: 'bg-red-500/10 text-red-700', warning: 'bg-amber-500/10 text-amber-700', info: 'bg-blue-500/10 text-blue-700' })[level]
  ?? 'bg-gray-500/10 text-gray-600'

const levelDot = (level) =>
  ({ error: 'bg-red-500', warning: 'bg-amber-500', info: 'bg-blue-500' })[level] ?? 'bg-gray-400'

const priorityBadge = (p) =>
  ({ critical: 'bg-red-500/15 text-red-700', high: 'bg-orange-500/15 text-orange-700',
     medium: 'bg-amber-500/15 text-amber-700', low: 'bg-blue-500/15 text-blue-700' })[p]
  ?? 'bg-gray-500/10 text-gray-600'

const envBadge = (env) =>
  ({ production: 'bg-red-500/10 text-red-700', staging: 'bg-amber-500/10 text-amber-700',
     development: 'bg-emerald-500/10 text-emerald-700' })[env?.toLowerCase()]
  ?? 'bg-gray-500/10 text-gray-600'

// ── Filter state — initialised from URL query ─────────────────────────────────
const tab         = ref(route.query.tab         ?? 'unresolved')
const search      = ref(route.query.search      ?? '')
const environment = ref(route.query.env         ?? '')
const release     = ref(route.query.release     ?? '')
const page        = ref(Number(route.query.page ?? 1))
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
    ...(tab.value !== 'unresolved' ? { tab: tab.value }         : {}),
    ...(search.value               ? { search: search.value }   : {}),
    ...(environment.value          ? { env: environment.value } : {}),
    ...(release.value              ? { release: release.value } : {}),
    ...(page.value > 1             ? { page: page.value }       : {}),
  }})
}

function fetchIssues() {
  store.fetch(route.params.id, activeStatus(), {
    search:      activeSearch(),
    environment: environment.value,
    release:     release.value,
    page:        page.value,
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
  page.value = 1
  selected.value = new Set()
  syncQuery()
  fetchIssues()
}

function refetch() {
  page.value = 1
  selected.value = new Set()
  syncQuery()
  fetchIssues()
}

function onSearch() {
  clearTimeout(searchTimer)
  searchTimer = setTimeout(refetch, 300)
}

function goToPage(p) {
  if (p < 1 || p > store.totalPages || p === store.page) return
  page.value = p
  selected.value = new Set()
  syncQuery()
  fetchIssues()
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

// ── Pagination helpers ────────────────────────────────────────────────────────
const pageStart = computed(() => store.total === 0 ? 0 : (store.page - 1) * store.perPage + 1)
const pageEnd   = computed(() => Math.min(store.page * store.perPage, store.total))

// Builds the page button list with ellipsis, e.g. [1, '...', 4, 5, 6, '...', 12]
const pageButtons = computed(() => {
  const total = store.totalPages
  const cur   = store.page
  if (total <= 7) return Array.from({ length: total }, (_, i) => i + 1)

  const pages = new Set([1, total, cur - 1, cur, cur + 1].filter(p => p >= 1 && p <= total))
  const sorted = [...pages].sort((a, b) => a - b)
  const result = []
  let prev = 0
  for (const p of sorted) {
    if (p - prev > 1) result.push('...')
    result.push(p)
    prev = p
  }
  return result
})

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
/* Hallmark · macrostructure: Ledger Row (shared with ProjectsView) · genre: modern-minimal (formal/exclusive)
 * theme: DevPulse locked system (design.md) — formal white workspace, oxblood accent
 * display: Fraunces · body: Geist · outlier(mono): JetBrains Mono
 * hairline dividers, no boxed table panel — see design.md § Macrostructure family
 */
.issues-root {
  background: var(--dp-paper);
  font-family: var(--dp-font-body);
  color: var(--dp-ink);
}

.dp-display {
  font-family: var(--dp-font-display);
  letter-spacing: -0.01em;
}

.dp-mono {
  font-family: var(--dp-font-mono);
  font-variant-numeric: tabular-nums;
}

.dv-dropdown {
  box-shadow: 0 8px 24px oklch(20% 0.02 40 / 14%);
}

.dropdown-enter-active { transition: opacity 0.1s ease, transform 0.1s ease; }
.dropdown-leave-active { transition: opacity 0.08s ease, transform 0.08s ease; }
.dropdown-enter-from, .dropdown-leave-to { opacity: 0; transform: translateY(-4px) scale(0.97); }

.bulk-bar-enter-active { transition: opacity 0.15s ease, transform 0.15s ease; }
.bulk-bar-leave-active { transition: opacity 0.1s ease, transform 0.1s ease; }
.bulk-bar-enter-from, .bulk-bar-leave-to { opacity: 0; transform: translateY(-6px); }

@media (prefers-reduced-motion: reduce) {
  .dropdown-enter-active, .dropdown-leave-active,
  .bulk-bar-enter-active, .bulk-bar-leave-active { transition-duration: 0.01s; }
}
</style>
