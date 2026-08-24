<template>
  <div class="issues-root flex-1 w-full">
    <div class="px-6 py-8">

      <!-- Header -->
      <div class="flex flex-wrap items-center justify-between gap-3 mb-5">
        <div class="flex items-center gap-3">
          <button
            @click="router.back()"
            type="button"
            title="Back"
            class="w-8 h-8 flex items-center justify-center rounded-lg text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)] transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)] shrink-0"
          >
            <ArrowLeft :size="16" />
          </button>
          <Breadcrumb>
            <BreadcrumbList>
              <BreadcrumbItem>
                <BreadcrumbLink as-child>
                  <router-link to="/projects">Projects</router-link>
                </BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator />
              <BreadcrumbItem>
                <BreadcrumbPage>Issues</BreadcrumbPage>
              </BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb>
          <span class="w-px h-4 bg-[var(--dp-rule)]" />
          <router-link
            :to="`/projects/${route.params.id}/releases`"
            class="text-[13px] text-[var(--dp-ink-3)] hover:text-[var(--dp-accent)] transition-colors flex items-center gap-1.5"
          >
            <History :size="13" />
            Releases
          </router-link>
        </div>

        <!-- Status / view tabs -->
        <div class="flex items-center bg-[var(--dp-surface-2)] border border-[var(--dp-rule)] rounded-xl p-1 gap-0.5">
          <button
            v-for="t in TABS"
            :key="t.value"
            @click="setTab(t.value)"
            :class="tab === t.value ? 'bg-[var(--dp-accent)] text-white' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)]'"
            class="px-3 py-1 rounded-lg text-xs font-medium capitalize transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
          >
            {{ t.label }}
          </button>
        </div>
      </div>

      <!-- Stats — bento cards, each with its own read on what the number means -->
      <div v-if="store.stats" class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-5">
        <div v-for="s in statCards" :key="s.key"
          class="rounded-2xl border border-[var(--dp-rule)] bg-[var(--dp-surface)] p-4 flex flex-col gap-2">
          <div class="flex items-center gap-1.5" :class="s.color">
            <component :is="s.icon" :size="14" />
            <span class="text-[10px] uppercase tracking-wide font-medium text-[var(--dp-ink-3)]">{{ s.label }}</span>
          </div>
          <span class="dp-mono tabular-nums text-2xl font-semibold" :class="s.color">{{ s.value }}</span>
          <p class="text-[11px] text-[var(--dp-ink-3)] leading-snug">{{ s.story }}</p>
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
          class="flex-1 min-w-[10rem] bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-xl px-4 py-2.5 text-sm text-[var(--dp-ink)]
                 placeholder-[var(--dp-ink-3)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors"
        />
        <div v-else class="flex-1 min-w-[10rem] flex items-center gap-2 bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-xl px-4 py-2.5">
          <span class="text-xs text-[var(--dp-accent)] font-medium">Performance vitals</span>
          <span class="text-[10px] text-[var(--dp-ink-3)]">· Web Core Vitals events</span>
        </div>
        <div class="relative" ref="filterPanelEl">
          <button type="button" @click="toggleFilters"
            class="flex items-center gap-2 rounded-xl border border-[var(--dp-rule)] bg-[var(--dp-surface)] px-4 py-2.5 text-sm text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] transition-colors"
            :class="{ 'border-[var(--dp-accent)] text-[var(--dp-accent)]': activeFilterCount > 0 }">
            <SlidersHorizontal :size="14" />
            Filters
            <span v-if="activeFilterCount" class="dp-mono text-[10px] font-bold bg-[var(--dp-accent-soft)] text-[var(--dp-accent)] rounded-full px-1.5 py-0.5 tabular-nums">
              {{ activeFilterCount }}
            </span>
            <ChevronDown :size="14" class="transition-transform" :class="{ 'rotate-180': showFilters }" />
          </button>

          <Transition name="dropdown">
            <div v-if="showFilters"
              class="dv-dropdown absolute right-0 top-full mt-1.5 w-64 max-w-[calc(100vw-2rem)] bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-xl z-50 p-4 space-y-3 text-left">
              <div>
                <label class="dp-label">Environment</label>
                <Select v-model="environmentModel">
                  <SelectTrigger class="dp-select-trigger">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent class="bg-[var(--dp-surface)] border-[var(--dp-rule)] text-[var(--dp-ink)]">
                    <SelectItem value="all" class="dp-select-item">All envs</SelectItem>
                    <SelectItem v-for="env in ENVIRONMENTS" :key="env" :value="env" class="dp-select-item capitalize">{{ env }}</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <div>
                <label class="dp-label">Version</label>
                <Select v-model="releaseModel">
                  <SelectTrigger class="dp-select-trigger">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent class="bg-[var(--dp-surface)] border-[var(--dp-rule)] text-[var(--dp-ink)]">
                    <SelectItem value="all" class="dp-select-item">All versions</SelectItem>
                    <SelectItem v-for="r in releases" :key="r.id" :value="r.version" class="dp-select-item">v{{ r.version }}</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <button v-if="activeFilterCount" @click="clearFilters"
                class="text-xs text-[var(--dp-ink-3)] hover:text-[var(--dp-ink-2)] transition-colors">
                Clear filters
              </button>
            </div>
          </Transition>
        </div>
      </div>

      <!-- Bulk action bar -->
      <Transition name="bulk-bar">
        <div
          v-if="selected.size > 0"
          class="flex items-center gap-3 bg-[var(--dp-accent-soft)] border border-[var(--dp-accent)]/25 rounded-xl px-4 py-2.5 mb-3"
        >
          <span class="text-sm text-[var(--dp-accent)] font-medium">{{ selected.size }} selected</span>
          <div class="flex gap-2 ml-auto">
            <button
              @click="bulkResolve"
              class="text-xs px-3 py-1.5 rounded-lg bg-emerald-500/10 text-emerald-700
                     hover:bg-emerald-500/15 transition-colors font-medium"
            >
              Resolve
            </button>
            <button
              @click="bulkIgnore"
              class="text-xs px-3 py-1.5 rounded-lg bg-[var(--dp-surface-2)] text-[var(--dp-ink-2)]
                     hover:bg-[var(--dp-rule)] transition-colors"
            >
              Ignore
            </button>
            <button
              @click="selected = new Set()"
              class="text-xs text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] px-2 transition-colors"
            >
              Clear
            </button>
          </div>
        </div>
      </Transition>

      <!-- Loading skeleton -->
      <div v-if="store.loading" class="space-y-px rounded-2xl overflow-hidden border border-[var(--dp-rule)]">
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

      <!-- Boxed table — matches the reference DataTable: rounded-2xl border, washed header -->
      <div v-else class="overflow-hidden rounded-2xl border border-[var(--dp-rule)] bg-[var(--dp-surface)]">

        <!-- Table header — 3 cols on mobile (checkbox/issue/menu), full 6 from md -->
        <div class="grid grid-cols-[40px_1fr_36px] md:grid-cols-[40px_1fr_80px_80px_96px_36px] gap-0 border-b border-[var(--dp-rule)] bg-[var(--dp-surface-2)]/60
                    text-xs font-medium text-[var(--dp-ink-3)]">
          <div class="flex items-center justify-center py-3">
            <Checkbox
              :checked="allSelected ? true : (someSelected ? 'indeterminate' : false)"
              @update:checked="toggleAll"
            />
          </div>
          <button type="button" @click="handleSort('title')"
            class="px-4 py-3 inline-flex items-center gap-1 text-left hover:text-[var(--dp-accent)] transition-colors">
            Issue
            <ArrowUp v-if="sortKey === 'title' && sortDirection === 'asc'" :size="12" />
            <ArrowDown v-else-if="sortKey === 'title' && sortDirection === 'desc'" :size="12" />
            <ArrowUpDown v-else :size="12" class="text-[var(--dp-rule)]" />
          </button>
          <button type="button" @click="handleSort('priority')"
            class="hidden md:inline-flex items-center justify-center gap-1 py-3 hover:text-[var(--dp-accent)] transition-colors">
            Priority
            <ArrowUp v-if="sortKey === 'priority' && sortDirection === 'asc'" :size="12" />
            <ArrowDown v-else-if="sortKey === 'priority' && sortDirection === 'desc'" :size="12" />
            <ArrowUpDown v-else :size="12" class="text-[var(--dp-rule)]" />
          </button>
          <button type="button" @click="handleSort('event_count')"
            class="hidden md:inline-flex items-center justify-end gap-1 pr-3 py-3 hover:text-[var(--dp-accent)] transition-colors">
            Events
            <ArrowUp v-if="sortKey === 'event_count' && sortDirection === 'asc'" :size="12" />
            <ArrowDown v-else-if="sortKey === 'event_count' && sortDirection === 'desc'" :size="12" />
            <ArrowUpDown v-else :size="12" class="text-[var(--dp-rule)]" />
          </button>
          <button type="button" @click="handleSort('last_seen')"
            class="hidden md:inline-flex items-center justify-end gap-1 pr-3 py-3 hover:text-[var(--dp-accent)] transition-colors">
            Last seen
            <ArrowUp v-if="sortKey === 'last_seen' && sortDirection === 'asc'" :size="12" />
            <ArrowDown v-else-if="sortKey === 'last_seen' && sortDirection === 'desc'" :size="12" />
            <ArrowUpDown v-else :size="12" class="text-[var(--dp-rule)]" />
          </button>
          <span></span>
        </div>

        <!-- Rows -->
        <div class="divide-y divide-[var(--dp-rule)]">
          <div
            v-for="issue in sortedIssues"
            :key="issue.id"
            class="grid grid-cols-[40px_1fr_36px] md:grid-cols-[40px_1fr_80px_80px_96px_36px] gap-0 items-center hover:bg-[var(--dp-surface-2)]/60 transition-colors group relative border-l-2"
            :class="[selected.has(issue.id) ? 'bg-[var(--dp-accent-soft)]/50' : 'bg-[var(--dp-surface)]', levelBorder(issue.level)]"
          >
            <!-- Checkbox -->
            <div class="flex items-center justify-center py-3.5" @click.stop>
              <Checkbox
                :checked="selected.has(issue.id)"
                @update:checked="toggleSelect(issue.id)"
              />
            </div>

            <!-- Title + level (+ mobile-only meta row) -->
            <router-link
              :to="`/issues/${issue.id}`"
              class="flex flex-col md:flex-row md:items-center gap-1 md:gap-2.5 px-4 py-3.5 min-w-0"
            >
              <div class="flex items-center gap-2.5 min-w-0">
                <span :class="levelBadge(issue.level)"
                      class="text-[10px] font-bold px-2 py-1 rounded uppercase tracking-wider shrink-0 hidden sm:inline">
                  {{ issue.level }}
                </span>
                <p class="text-[13px] text-[var(--dp-ink)] truncate group-hover:text-[var(--dp-accent)] transition-colors leading-snug">
                  {{ issue.title }}
                </p>
                <span v-if="issue.environment && issue.environment !== 'production'"
                      :class="envBadge(issue.environment)"
                      class="text-[10px] font-bold px-2 py-1 rounded uppercase tracking-wider shrink-0 hidden md:inline">
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
            <div class="flex items-center justify-center py-3.5 shrink-0">
              <DropdownMenu @update:open="(open) => { if (!open) menuConfirm = null }">
                <DropdownMenuTrigger as-child>
                  <button
                    class="w-6 h-6 flex items-center justify-center rounded-lg text-[var(--dp-ink-3)]
                           hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)] transition-colors opacity-0 group-hover:opacity-100 focus-visible:opacity-100 focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)] data-[state=open]:opacity-100 data-[state=open]:text-[var(--dp-ink)] data-[state=open]:bg-[var(--dp-surface-2)]"
                    title="Actions"
                  >
                    <MoreVertical :size="14" />
                  </button>
                </DropdownMenuTrigger>
                <DropdownMenuContent class="w-44">
                  <DropdownMenuItem
                    @select="onMenuSelect($event, 'resolve', issue.id)"
                    :class="isPending(issue.id, 'resolve') ? 'bg-emerald-500/15 text-emerald-700 font-semibold' : ''"
                  >
                    <Check :size="13" class="shrink-0" />
                    {{ isPending(issue.id, 'resolve') ? 'Confirm resolve?' : 'Resolve' }}
                  </DropdownMenuItem>
                  <DropdownMenuItem
                    @select="onMenuSelect($event, 'ignore', issue.id)"
                    :class="isPending(issue.id, 'ignore') ? 'bg-red-500/10 text-red-700 font-semibold' : ''"
                  >
                    <X :size="13" class="shrink-0" />
                    {{ isPending(issue.id, 'ignore') ? 'Confirm ignore?' : 'Ignore' }}
                  </DropdownMenuItem>
                  <template v-if="isPending(issue.id, 'resolve') || isPending(issue.id, 'ignore')">
                    <DropdownMenuSeparator />
                    <DropdownMenuItem @select="menuConfirm = null">
                      Cancel
                    </DropdownMenuItem>
                  </template>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>

          </div>
        </div>

        <!-- Pagination footer -->
        <div class="px-4 py-3 border-t border-[var(--dp-rule)] flex items-center justify-between gap-4">

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
              class="w-7 h-7 flex items-center justify-center rounded-lg text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-rule)]
                     disabled:opacity-30 disabled:cursor-not-allowed transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
              aria-label="Previous page"
            >
              <ChevronLeft :size="14" />
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
                  ? 'text-[var(--dp-accent)] font-semibold'
                  : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-rule)]'"
                class="dp-mono w-7 h-7 flex items-center justify-center rounded-lg text-xs transition-colors tabular-nums focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
              >
                {{ p }}
              </button>
            </span>

            <!-- Next -->
            <button
              @click="goToPage(store.page + 1)"
              :disabled="store.page >= store.totalPages"
              class="w-7 h-7 flex items-center justify-center rounded-lg text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-rule)]
                     disabled:opacity-30 disabled:cursor-not-allowed transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]"
              aria-label="Next page"
            >
              <ChevronRight :size="14" />
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
import { onClickOutside } from '@vueuse/core'
import { useRoute, useRouter } from 'vue-router'
import axios from 'axios'
import {
  ArrowUp, ArrowDown, ArrowUpDown, ArrowLeft, History,
  AlertCircle, Sparkles, RotateCcw, Activity,
  ChevronLeft, ChevronRight, MoreVertical, Check, X,
  SlidersHorizontal, ChevronDown,
} from 'lucide-vue-next'
import { useIssuesStore } from '../stores/issues'
import { useToastStore }  from '../stores/toast'
import { ENVIRONMENTS } from '../composables/useColors'
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@/components/ui/breadcrumb'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Checkbox } from '@/components/ui/checkbox'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'

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

const levelBorder = (level) =>
  ({ error: 'border-l-red-500', warning: 'border-l-amber-500', info: 'border-l-blue-500' })[level] ?? 'border-l-transparent'

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
const menuConfirm = ref(null) // { id, type } — pending confirmation for ellipsis action
const selected    = ref(new Set())
const showFilters   = ref(false)
const filterPanelEl = ref(null)

let searchTimer = null

// ── Custom Select filters — reka-ui Select reserves "" internally, so the
// UI model uses the sentinel "all" and translates to/from the real "" filter value.
const environmentModel = computed({
  get: () => environment.value || 'all',
  set: (v) => { environment.value = v === 'all' ? '' : v; refetch() },
})
const releaseModel = computed({
  get: () => release.value || 'all',
  set: (v) => { release.value = v === 'all' ? '' : v; refetch() },
})

// ── Sortable columns — sorts the currently-loaded page only (server already
// paginates); matches the same client-side sort used on the Projects table.
const sortKey       = ref(null)
const sortDirection = ref('asc')
const PRIORITY_RANK = { critical: 4, high: 3, medium: 2, low: 1 }

function handleSort(key) {
  sortDirection.value = sortKey.value === key && sortDirection.value === 'asc' ? 'desc' : 'asc'
  sortKey.value = key
}

const sortedIssues = computed(() => {
  if (!sortKey.value) return store.issues
  const dir = sortDirection.value === 'asc' ? 1 : -1

  return [...store.issues].sort((a, b) => {
    if (sortKey.value === 'title') return a.title.localeCompare(b.title) * dir
    if (sortKey.value === 'priority') {
      return ((PRIORITY_RANK[a.priority] ?? 0) - (PRIORITY_RANK[b.priority] ?? 0)) * dir
    }
    if (sortKey.value === 'event_count') return (a.event_count - b.event_count) * dir
    // last_seen
    return (new Date(a.last_seen).getTime() - new Date(b.last_seen).getTime()) * dir
  })
})

// ── Stats — bento cards ────────────────────────────────────────────────────────
const statCards = computed(() => {
  if (!store.stats) return []
  return [
    { key: 'unresolved', label: 'Unresolved', icon: AlertCircle, color: 'text-[var(--dp-danger)]',
      value: store.stats.issues.unresolved, story: 'Open issues waiting on a fix across this project.' },
    { key: 'new24h', label: 'New 24h', icon: Sparkles, color: 'text-amber-600',
      value: store.stats.issues.new_24h, story: 'First-seen issues in the last day.' },
    { key: 'regressions', label: 'Regressions', icon: RotateCcw, color: 'text-orange-600',
      value: store.stats.issues.regressions_24h, story: 'Previously resolved issues that came back.' },
    { key: 'events24h', label: 'Events 24h', icon: Activity, color: 'text-[var(--dp-accent)]',
      value: store.stats.events_24h, story: 'Total error events ingested in the last day.' },
  ]
})

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
  store.fetchStats(route.params.id)
  fetchIssues()
  document.addEventListener('keydown', onKeydown)
  try {
    const { data } = await axios.get(`/api/projects/${route.params.id}/releases`)
    releases.value = data.data
  } catch {}
})

onUnmounted(() => {
  document.removeEventListener('keydown', onKeydown)
})

// ── Keyboard ──────────────────────────────────────────────────────────────────
function onKeydown(e) {
  if (e.key !== 'Escape') return
  showFilters.value = false
}

// ── Filter panel ────────────────────────────────────────────────────────────────
// vueuse's onClickOutside (not a hand-rolled document listener) — the shadcn
// Select inside this panel opens on pointerdown and can shift layout (body
// scroll lock) between mousedown and mouseup, which defeats a naive
// `.contains(e.target)` check on the plain 'click' event. onClickOutside
// is hardened against exactly that class of popover interaction.
function toggleFilters() { showFilters.value = !showFilters.value }
onClickOutside(filterPanelEl, () => { showFilters.value = false })

const activeFilterCount = computed(() =>
  (environment.value ? 1 : 0) + (release.value ? 1 : 0)
)

function clearFilters() {
  environment.value = ''
  release.value     = ''
  refetch()
}
function isPending(id, type) {
  return menuConfirm.value?.id === id && menuConfirm.value?.type === type
}

async function action(type, id) {
  if (isPending(id, type)) {
    menuConfirm.value = null
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

// First select arms the confirmation and keeps the menu open (preventDefault
// stops reka-ui's default close-on-select); second select confirms and lets
// the menu close normally.
function onMenuSelect(event, type, id) {
  if (!isPending(id, type)) event.preventDefault()
  action(type, id)
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
/* Hallmark · macrostructure: boxed table (shared with ProjectsView) · genre: modern-minimal (clean/formal)
 * theme: DevPulse locked system (design.md) — slate/maroon workspace (accent moved off the teleradiology-matched indigo per a later request)
 * body: Inter · outlier(mono): JetBrains Mono
 * rows live in a rounded-2xl bordered box — see design.md § Macrostructure family
 * shared components (Breadcrumb, Select, Checkbox) ported from teleradiology's own
 * ui/ kit — same reka-ui + lucide-vue-next stack, adapted to --dp-* tokens
 * stats: bento cards, not a flat label:value strip · sort: client-side on the
 * currently-loaded page, same convention as ProjectsView's sortable headers
 */
.issues-root {
  background: var(--dp-paper);
  font-family: var(--dp-font-body);
  color: var(--dp-ink);
}

.dp-mono {
  font-family: var(--dp-font-mono);
  font-variant-numeric: tabular-nums;
}

.dv-dropdown {
  box-shadow: 0 8px 24px oklch(20% 0.02 40 / 14%);
}

.dp-label {
  display: block;
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--dp-ink-2);
  margin-bottom: 0.375rem;
}

/* shadcn Select styled to match the rest of this filter panel's fields */
.dp-select-trigger {
  width: 100%;
  background: var(--dp-paper);
  border-color: var(--dp-rule);
  border-radius: 0.75rem;
  padding: 0.625rem 0.75rem;
  height: auto;
  font-size: 0.875rem;
  color: var(--dp-ink);
}

.dp-select-item:focus {
  background: var(--dp-accent-soft);
  color: var(--dp-accent);
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
