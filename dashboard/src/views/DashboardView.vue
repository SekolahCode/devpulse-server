<template>
  <div class="home-root flex-1 w-full">
    <div class="px-6 py-10 space-y-8">

      <!-- Page header -->
      <div>
        <h1 class="text-2xl md:text-[28px] font-semibold text-[var(--dp-ink)] tracking-tight">Dashboard</h1>
        <p class="text-sm text-[var(--dp-ink-2)] mt-1">Welcome back — here's what's happening across your projects.</p>
      </div>

      <!-- Stats — same bento language as the Issues page, global across every project -->
      <div v-if="stats" class="grid grid-cols-2 md:grid-cols-4 gap-3">
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
      <div v-else class="grid grid-cols-2 md:grid-cols-4 gap-3">
        <div v-for="i in 4" :key="i" class="h-[6.5rem] rounded-2xl bg-[var(--dp-surface-2)] animate-pulse" />
      </div>

      <!-- Two-column: projects snapshot (wide) + recent activity (narrow) -->
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-4">

        <!-- Your Projects -->
        <div class="lg:col-span-8 rounded-2xl border border-[var(--dp-rule)] bg-[var(--dp-surface)] overflow-hidden">
          <div class="flex items-center justify-between px-5 py-4 border-b border-[var(--dp-rule)]">
            <div>
              <h2 class="text-[15px] font-semibold text-[var(--dp-ink)]">Your projects</h2>
              <p class="text-xs text-[var(--dp-ink-2)] mt-0.5">Sorted by unresolved issues</p>
            </div>
            <router-link to="/projects"
              class="flex items-center gap-1 text-xs font-medium text-[var(--dp-accent)] hover:opacity-80 transition-opacity">
              View all
              <ArrowRight :size="12" />
            </router-link>
          </div>

          <div v-if="!projectStore.loaded" class="divide-y divide-[var(--dp-rule)]">
            <div v-for="i in 4" :key="i" class="h-16 px-5 flex items-center gap-3">
              <div class="w-8 h-8 rounded-full bg-[var(--dp-surface-2)] animate-pulse shrink-0" />
              <div class="h-3.5 w-40 rounded bg-[var(--dp-surface-2)] animate-pulse" />
            </div>
          </div>

          <div v-else-if="!topProjects.length" class="flex flex-col items-center justify-center py-16 text-center">
            <p class="text-sm text-[var(--dp-ink)] font-medium">No projects yet</p>
            <router-link to="/projects" class="text-xs text-[var(--dp-accent)] hover:opacity-80 transition-opacity mt-1">
              Create your first project
            </router-link>
          </div>

          <div v-else class="divide-y divide-[var(--dp-rule)]">
            <router-link
              v-for="p in topProjects"
              :key="p.id"
              :to="`/projects/${p.id}/issues`"
              class="flex items-center gap-3 px-5 py-3.5 hover:bg-[var(--dp-surface-2)]/40 transition-colors"
            >
              <img :src="platformIcon(p.platform)" :alt="p.platform" class="w-8 h-8 rounded-full shrink-0" />
              <div class="min-w-0 flex-1">
                <p class="text-sm font-medium text-[var(--dp-ink)] truncate">{{ p.name }}</p>
                <p class="text-xs text-[var(--dp-ink-3)] capitalize">{{ p.platform }}</p>
              </div>
              <span v-if="health[p.id]?.total > 0"
                class="dp-mono inline-flex items-center gap-1 text-[12px] font-medium text-[var(--dp-danger)] bg-[var(--dp-danger-soft)] px-2 py-0.5 rounded-full tabular-nums shrink-0">
                ▲ {{ health[p.id].total }}
              </span>
              <span v-else class="dp-mono text-[12px] text-[var(--dp-ink-3)] tabular-nums shrink-0">quiet</span>
            </router-link>
          </div>
        </div>

        <!-- Recent Activity -->
        <div class="lg:col-span-4 rounded-2xl border border-[var(--dp-rule)] bg-[var(--dp-surface)] overflow-hidden">
          <div class="px-5 py-4 border-b border-[var(--dp-rule)]">
            <h2 class="text-[15px] font-semibold text-[var(--dp-ink)]">Recent activity</h2>
            <p class="text-xs text-[var(--dp-ink-2)] mt-0.5">Live, as events come in</p>
          </div>

          <div v-if="!issuesStore.live.length" class="flex flex-col items-center justify-center py-16 text-center px-5">
            <Activity :size="20" class="text-[var(--dp-ink-3)] mb-2" />
            <p class="text-sm text-[var(--dp-ink-2)]">No recent activity yet</p>
            <p class="text-xs text-[var(--dp-ink-3)] mt-1">New events will appear here in real time.</p>
          </div>

          <div v-else class="divide-y divide-[var(--dp-rule)] max-h-[26rem] overflow-y-auto">
            <div v-for="event in issuesStore.live" :key="`${event.issue_id}-${event.ts}`" class="px-5 py-3">
              <div class="flex items-center gap-2 mb-1">
                <span :class="levelDot(event.level)" class="w-1.5 h-1.5 rounded-full shrink-0" />
                <span class="text-[11px] text-[var(--dp-ink-3)] truncate">{{ projectName(event.project_id) }}</span>
                <span v-if="event.is_regression" class="text-[9px] font-bold px-1.5 py-0.5 rounded uppercase tracking-wide bg-orange-500/10 text-orange-700 shrink-0">
                  Regression
                </span>
                <span class="dp-mono text-[10px] text-[var(--dp-ink-3)] tabular-nums ml-auto shrink-0">{{ timeAgo(event.ts) }}</span>
              </div>
              <p class="text-[13px] text-[var(--dp-ink)] truncate">{{ event.title }}</p>
            </div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup>
import { computed, onMounted, reactive } from 'vue'
import axios from 'axios'
import { AlertCircle, Sparkles, RotateCcw, Activity, ArrowRight } from 'lucide-vue-next'
import { useProjectStore } from '../stores/project'
import { useIssuesStore } from '../stores/issues'
import { platformIcon } from '../composables/useColors'

const projectStore = useProjectStore()
const issuesStore  = useIssuesStore()

const stats = computed(() => issuesStore.stats)

const statCards = computed(() => {
  if (!stats.value) return []
  return [
    { key: 'unresolved', label: 'Unresolved', icon: AlertCircle, color: 'text-[var(--dp-danger)]',
      value: stats.value.issues.unresolved, story: 'Open issues waiting on a fix across every project.' },
    { key: 'new24h', label: 'New 24h', icon: Sparkles, color: 'text-amber-600',
      value: stats.value.issues.new_24h, story: 'First-seen issues in the last day.' },
    { key: 'regressions', label: 'Regressions', icon: RotateCcw, color: 'text-orange-600',
      value: stats.value.issues.regressions_24h, story: 'Previously resolved issues that came back.' },
    { key: 'events24h', label: 'Events 24h', icon: Activity, color: 'text-[var(--dp-accent)]',
      value: stats.value.events_24h, story: 'Total error events ingested in the last day.' },
  ]
})

// ── Per-project health (unresolved count) — same lightweight check ProjectsView
// uses: total = unresolved count from /api/issues?limit=1, no backend changes.
const health = reactive({})

async function loadHealth(projects) {
  await Promise.all(projects.map(async (p) => {
    health[p.id] = { state: 'loading' }
    try {
      const { data } = await axios.get('/api/issues', { params: { project_id: p.id, limit: 1 } })
      health[p.id] = { state: 'done', total: data.total ?? 0, lastSeen: data.data?.[0]?.last_seen ?? null }
    } catch {
      health[p.id] = { state: 'error' }
    }
  }))
}

const topProjects = computed(() =>
  [...projectStore.projects]
    .sort((a, b) => (health[b.id]?.total ?? 0) - (health[a.id]?.total ?? 0))
    .slice(0, 5)
)

function projectName(projectId) {
  return projectStore.projects.find(p => p.id === projectId)?.name ?? 'Unknown project'
}

const levelDot = (level) =>
  ({ error: 'bg-red-500', warning: 'bg-amber-500', info: 'bg-blue-500' })[level] ?? 'bg-gray-400'

function timeAgo(ts) {
  const s = (Date.now() - ts) / 1000
  if (s < 60)    return 'just now'
  if (s < 3600)  return `${Math.floor(s / 60)}m ago`
  return `${Math.floor(s / 3600)}h ago`
}

onMounted(async () => {
  issuesStore.fetchStats()
  try {
    await projectStore.load()
    loadHealth(projectStore.projects)
  } catch { /* projects list failed to load — snapshot stays empty, no invented rows */ }
})
</script>

<style scoped>
/* Hallmark · macrostructure: landing/overview (distinct from Analytics' chart
 * briefing and Projects' boxed table) · genre: modern-minimal (clean/formal)
 * theme: DevPulse locked system (design.md) — slate/maroon workspace
 * body: Inter · outlier(mono): JetBrains Mono
 * stat bento (shared language with Issues) + projects snapshot + live
 * recent-activity feed sourced straight from the existing WS event stream —
 * no invented metrics, see design.md § Data honesty
 */
.home-root {
  background: var(--dp-paper);
  font-family: var(--dp-font-body);
  color: var(--dp-ink);
}

.dp-mono {
  font-family: var(--dp-font-mono);
  font-variant-numeric: tabular-nums;
}
</style>
