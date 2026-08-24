<template>
  <div class="analytics-root flex-1 w-full">
    <div class="px-6 py-10 space-y-10">

      <!-- Page header -->
      <div class="flex flex-wrap items-end justify-between gap-4">
        <div>
          <h1 class="text-2xl md:text-[28px] font-semibold text-[var(--dp-ink)] tracking-tight">Analytics</h1>
          <p class="text-sm text-[var(--dp-ink-2)] mt-1">Last 14 days · all projects</p>
        </div>

        <!-- Time range selector using shadcn Select -->
        <Select v-model="range">
          <SelectTrigger class="w-36 rounded-xl bg-[var(--dp-surface)] border-[var(--dp-rule)] text-[var(--dp-ink)] text-sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent class="bg-[var(--dp-surface)] border-[var(--dp-rule)] text-[var(--dp-ink)]">
            <SelectItem value="14" class="focus:bg-[var(--dp-accent-soft)] focus:text-[var(--dp-accent)] [&_svg]:text-[var(--dp-ink-3)]">Last 14 days</SelectItem>
            <SelectItem value="7" class="focus:bg-[var(--dp-accent-soft)] focus:text-[var(--dp-accent)] [&_svg]:text-[var(--dp-ink-3)]">Last 7 days</SelectItem>
            <SelectItem value="30" class="focus:bg-[var(--dp-accent-soft)] focus:text-[var(--dp-accent)] [&_svg]:text-[var(--dp-ink-3)]">Last 30 days</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <!-- Briefing lead: lead metric + supporting stats, hairline-framed, no shadow -->
      <Card class="rounded-2xl shadow-none px-0 py-0 gap-0 bg-[var(--dp-surface)] text-[var(--dp-ink)] border-[var(--dp-rule)]">
        <template v-if="loading">
          <div class="flex flex-col lg:flex-row">
            <div class="flex-1 lg:max-w-[58%] p-6 lg:p-8 space-y-3">
              <Skeleton class="h-3 w-20" />
              <Skeleton class="h-16 w-48" />
              <Skeleton class="h-3 w-64" />
            </div>
            <div class="flex-1 grid grid-cols-3 lg:grid-cols-1">
              <Skeleton v-for="i in 3" :key="i" class="h-16 m-4 rounded-xl" />
            </div>
          </div>
        </template>

        <template v-else>
          <div class="flex flex-col lg:flex-row divide-y lg:divide-y-0 lg:divide-x divide-[var(--dp-rule)]">

            <!-- Lead metric: unresolved issues -->
            <div class="flex-1 lg:max-w-[58%] p-6 lg:p-8 flex flex-col justify-center gap-2">
              <p class="text-[11px] uppercase tracking-[0.14em] text-[var(--dp-ink-2)] font-medium">Unresolved</p>
              <p class="dv-mono tabular-nums text-[var(--dp-danger)] leading-none font-medium"
                style="font-size: clamp(3rem, 3.2vw + 1.75rem, 5.5rem)">
                {{ heroDisplay }}
              </p>
              <p class="text-sm text-[var(--dp-ink-2)] max-w-sm">
                open issues across every project you're tracking — investigate before they age.
              </p>
            </div>

            <!-- Supporting stats -->
            <div class="flex-1 grid grid-cols-3 lg:grid-cols-1 divide-x lg:divide-x-0 lg:divide-y divide-[var(--dp-rule)]">
              <div
                v-for="s in secondaryStats" :key="s.key"
                class="p-5 lg:px-8 lg:py-5 flex flex-col lg:flex-row lg:items-center lg:justify-between gap-1.5"
              >
                <span class="text-[11px] uppercase tracking-widest text-[var(--dp-ink-2)] font-medium">{{ s.label }}</span>
                <span class="dv-mono tabular-nums text-xl lg:text-2xl font-medium" :class="s.color">{{ s.display }}</span>
              </div>
            </div>
          </div>
        </template>
      </Card>

      <!-- Charts — row 1: timeline (wide) + level breakdown (narrow) -->
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-4">

        <Card class="lg:col-span-8 rounded-2xl shadow-none px-6 py-5 gap-4 bg-[var(--dp-surface)] text-[var(--dp-ink)] border-[var(--dp-rule)]">
          <CardHeader class="p-0">
            <CardTitle class="text-[15px] font-semibold text-[var(--dp-ink)]">Activity timeline</CardTitle>
            <CardDescription class="text-[var(--dp-ink-2)]">Events ingested and new issues opened per day</CardDescription>
          </CardHeader>
          <CardContent class="p-0">
            <Skeleton v-if="loading" class="h-56 w-full rounded-xl" />
            <div v-else class="h-56">
              <Line :data="lineChartData" :options="lineChartOptions" />
            </div>
          </CardContent>
        </Card>

        <Card class="lg:col-span-4 rounded-2xl shadow-none px-6 py-5 gap-4 bg-[var(--dp-surface)] text-[var(--dp-ink)] border-[var(--dp-rule)]">
          <CardHeader class="p-0">
            <CardTitle class="text-[15px] font-semibold text-[var(--dp-ink)]">Issues by level</CardTitle>
            <CardDescription class="text-[var(--dp-ink-2)]">Unresolved issues breakdown</CardDescription>
          </CardHeader>
          <CardContent class="p-0 flex flex-col items-center gap-4">
            <Skeleton v-if="loading" class="h-40 w-40 rounded-full" />
            <template v-else>
              <div class="h-44 w-44">
                <Doughnut :data="levelDoughnutData" :options="doughnutOptions" />
              </div>
              <div class="flex flex-col gap-1.5 w-full">
                <div v-for="item in levelItems" :key="item.label"
                  class="flex items-center justify-between text-xs">
                  <div class="flex items-center gap-2">
                    <span class="w-2.5 h-2.5 rounded-sm shrink-0" :style="{ background: item.color }" />
                    <span class="text-[var(--dp-ink-2)] capitalize">{{ item.label }}</span>
                  </div>
                  <span class="dv-mono font-medium text-[var(--dp-ink)] tabular-nums">{{ item.count }}</span>
                </div>
              </div>
            </template>
          </CardContent>
        </Card>
      </div>

      <!-- Charts — row 2: status breakdown (narrow, left this time) + top projects (wide) -->
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-4">

        <Card class="lg:col-span-4 rounded-2xl shadow-none px-6 py-5 gap-4 bg-[var(--dp-surface)] text-[var(--dp-ink)] border-[var(--dp-rule)]">
          <CardHeader class="p-0">
            <CardTitle class="text-[15px] font-semibold text-[var(--dp-ink)]">Issue status</CardTitle>
            <CardDescription class="text-[var(--dp-ink-2)]">Distribution across all projects</CardDescription>
          </CardHeader>
          <CardContent class="p-0 flex flex-col items-center gap-4">
            <Skeleton v-if="loading" class="h-40 w-40 rounded-full" />
            <template v-else>
              <div class="h-44 w-44">
                <Doughnut :data="statusDoughnutData" :options="doughnutOptions" />
              </div>
              <div class="flex flex-col gap-1.5 w-full">
                <div v-for="item in statusItems" :key="item.label"
                  class="flex items-center justify-between text-xs">
                  <div class="flex items-center gap-2">
                    <span class="w-2.5 h-2.5 rounded-sm shrink-0" :style="{ background: item.color }" />
                    <span class="text-[var(--dp-ink-2)] capitalize">{{ item.label }}</span>
                  </div>
                  <span class="dv-mono font-medium text-[var(--dp-ink)] tabular-nums">{{ item.count }}</span>
                </div>
              </div>
            </template>
          </CardContent>
        </Card>

        <Card class="lg:col-span-8 rounded-2xl shadow-none px-6 py-5 gap-4 bg-[var(--dp-surface)] text-[var(--dp-ink)] border-[var(--dp-rule)]">
          <CardHeader class="p-0">
            <CardTitle class="text-[15px] font-semibold text-[var(--dp-ink)]">Top projects</CardTitle>
            <CardDescription class="text-[var(--dp-ink-2)]">Event volume by project over the last 7 days</CardDescription>
          </CardHeader>
          <CardContent class="p-0">
            <Skeleton v-if="loading" class="h-48 w-full rounded-xl" />
            <div v-else-if="!chartData?.top_projects?.length"
              class="h-48 flex items-center justify-center text-sm text-[var(--dp-ink-2)]">
              No event data yet
            </div>
            <div v-else class="h-48">
              <Bar :data="barChartData" :options="barChartOptions" />
            </div>
          </CardContent>
        </Card>
      </div>

    </div>
  </div>
</template>

<script setup>
import { computed, onMounted, reactive, ref } from 'vue'
import axios from 'axios'
import {
  Chart as ChartJS,
  CategoryScale, LinearScale,
  PointElement, LineElement,
  BarElement, ArcElement,
  Tooltip, Legend, Filler,
} from 'chart.js'
import { Line, Doughnut, Bar } from 'vue-chartjs'

import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Skeleton } from '@/components/ui/skeleton'

// ── Chart.js registration ────────────────────────────────────────────────────
ChartJS.register(
  CategoryScale, LinearScale,
  PointElement, LineElement,
  BarElement, ArcElement,
  Tooltip, Legend, Filler,
)

// ── State ────────────────────────────────────────────────────────────────────
const loading   = ref(true)
const stats     = ref(null)
const chartData = ref(null)
const range     = ref('14')

// ── Number-reveal count-up (hero + supporting stats) ────────────────────────
const prefersReducedMotion = typeof window !== 'undefined'
  && window.matchMedia('(prefers-reduced-motion: reduce)').matches

const counts = reactive({ unresolved: 0, new24h: 0, regressions: 0, events: 0 })

function animateNumber(setter, target, duration = 700) {
  if (prefersReducedMotion || !target) { setter(target || 0); return }
  const start = performance.now()
  function frame(now) {
    const t = Math.min(1, (now - start) / duration)
    const eased = 1 - Math.pow(1 - t, 3) // matches --ease-out
    setter(target * eased)
    if (t < 1) requestAnimationFrame(frame)
    else setter(target)
  }
  requestAnimationFrame(frame)
}

function triggerCountUp() {
  if (!stats.value) return
  animateNumber(v => (counts.unresolved  = v), stats.value.issues?.unresolved ?? 0, 900)
  animateNumber(v => (counts.new24h      = v), stats.value.issues?.new_24h ?? 0, 700)
  animateNumber(v => (counts.regressions = v), stats.value.issues?.regressions_24h ?? 0, 700)
  animateNumber(v => (counts.events      = v), stats.value.events_24h ?? 0, 700)
}

const heroDisplay = computed(() => Math.round(counts.unresolved).toLocaleString())

const secondaryStats = computed(() => [
  { key: 'new24h',      label: 'New 24h',      display: Math.round(counts.new24h).toLocaleString(),      color: 'text-amber-600'  },
  { key: 'regressions', label: 'Regressions',  display: Math.round(counts.regressions).toLocaleString(), color: 'text-orange-600' },
  { key: 'events',      label: 'Events 24h',   display: Math.round(counts.events).toLocaleString(),      color: 'text-[var(--dp-accent)]' },
])

// ── Data fetching ────────────────────────────────────────────────────────────
onMounted(async () => {
  const [statsRes, chartRes] = await Promise.all([
    axios.get('/api/stats').catch(() => null),
    axios.get('/api/stats/chart').catch(() => null),
  ])
  stats.value     = statsRes?.data ?? null
  chartData.value = chartRes?.data ?? null
  loading.value   = false
  triggerCountUp()
})

// ── Shared Chart.js theme (light canvas) ─────────────────────────────────────
const FONT_MONO = "'JetBrains Mono', ui-monospace, SFMono-Regular, monospace"

const C = {
  // Brand series — maroon, mirrors --dp-accent (Chart.js can't read oklch())
  violet:  '#800000',
  violetA: 'rgba(128,0,0,0.14)',
  red:     '#dc2626',
  redA:    'rgba(220,38,38,0.12)',
  amber:   '#d97706',
  emerald: '#059669',
  blue:    '#2563eb',
  gray:    '#9ca3af',
  // Grid/ticks sit directly on the light card surface now, warm-neutral tuned
  gridLine: 'rgba(30, 41, 59, 0.08)',
  tickText: '#64748b',
  // Tooltip stays a dark floating chip — pops regardless of page theme,
  // mirrors --dp-elevated-* (see design.md)
  tooltipBg:     '#1e293b',
  tooltipBorder: 'rgba(255,255,255,0.08)',
  tooltipTitle:  '#f1f5f9',
  tooltipBody:   '#cbd5e1',
}

const baseChartOptions = {
  responsive:          true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: C.tooltipBg,
      borderColor:     C.tooltipBorder,
      borderWidth:     1,
      titleColor:      C.tooltipTitle,
      titleFont:       { family: FONT_MONO, size: 11 },
      bodyColor:       C.tooltipBody,
      bodyFont:        { family: FONT_MONO, size: 11 },
      padding:         10,
      cornerRadius:    6,
    },
  },
}

// ── Line chart: events + new issues per day ───────────────────────────────────
const lineChartData = computed(() => {
  const eventsMap = Object.fromEntries(
    (chartData.value?.events_by_day ?? []).map(d => [d.day, d.count])
  )
  const issuesMap = Object.fromEntries(
    (chartData.value?.issues_by_day ?? []).map(d => [d.day, d.count])
  )

  const days = last14Days()
  return {
    labels: days.map(d => fmtDay(d)),
    datasets: [
      {
        label:           'Events',
        data:            days.map(d => eventsMap[d] ?? 0),
        borderColor:     C.violet,
        backgroundColor: C.violetA,
        borderWidth:     2,
        pointRadius:     3,
        pointHoverRadius: 5,
        fill:            true,
        tension:         0.35,
      },
      {
        label:           'New Issues',
        data:            days.map(d => issuesMap[d] ?? 0),
        borderColor:     C.red,
        backgroundColor: C.redA,
        borderWidth:     2,
        pointRadius:     3,
        pointHoverRadius: 5,
        fill:            true,
        tension:         0.35,
      },
    ],
  }
})

const lineChartOptions = {
  ...baseChartOptions,
  plugins: {
    ...baseChartOptions.plugins,
    legend: {
      display:  true,
      position: 'top',
      align:    'end',
      labels: {
        color:       C.tickText,
        boxWidth:    10,
        boxHeight:   10,
        borderRadius: 3,
        usePointStyle: false,
        font: { family: FONT_MONO, size: 11 },
        padding: 16,
      },
    },
  },
  scales: {
    x: {
      grid:  { color: C.gridLine },
      ticks: { color: C.tickText, font: { family: FONT_MONO, size: 11 }, maxRotation: 0 },
    },
    y: {
      grid:  { color: C.gridLine },
      ticks: { color: C.tickText, font: { family: FONT_MONO, size: 11 }, precision: 0 },
      beginAtZero: true,
    },
  },
}

// ── Doughnut: issues by level ────────────────────────────────────────────────
const LEVEL_COLORS = { error: C.red, warning: C.amber, info: C.blue }

const levelItems = computed(() => {
  const rows = chartData.value?.by_level ?? []
  return ['error', 'warning', 'info'].map(lvl => ({
    label: lvl,
    count: rows.find(r => r.level === lvl)?.count ?? 0,
    color: LEVEL_COLORS[lvl] ?? C.gray,
  }))
})

const levelDoughnutData = computed(() => ({
  labels:   levelItems.value.map(i => i.label),
  datasets: [{
    data:            levelItems.value.map(i => i.count),
    backgroundColor: levelItems.value.map(i => i.color),
    borderWidth:     0,
    hoverOffset:     4,
  }],
}))

// ── Bar chart: top projects ───────────────────────────────────────────────────
const barChartData = computed(() => {
  const projects = chartData.value?.top_projects ?? []
  return {
    labels: projects.map(p => p.name),
    datasets: [{
      label:           'Events (7d)',
      data:            projects.map(p => p.count),
      backgroundColor: C.violet,
      hoverBackgroundColor: '#5c0000',
      borderRadius:    6,
      borderSkipped:   false,
    }],
  }
})

const barChartOptions = {
  ...baseChartOptions,
  scales: {
    x: {
      grid:  { display: false },
      ticks: { color: C.tickText, font: { family: FONT_MONO, size: 11 } },
    },
    y: {
      grid:  { color: C.gridLine },
      ticks: { color: C.tickText, font: { family: FONT_MONO, size: 11 }, precision: 0 },
      beginAtZero: true,
    },
  },
}

// ── Doughnut: issues by status ───────────────────────────────────────────────
const STATUS_COLORS = { unresolved: C.red, resolved: C.emerald, ignored: C.gray }

const statusItems = computed(() => {
  const rows = chartData.value?.by_status ?? []
  return ['unresolved', 'resolved', 'ignored'].map(s => ({
    label: s,
    count: rows.find(r => r.status === s)?.count ?? 0,
    color: STATUS_COLORS[s] ?? C.gray,
  }))
})

const statusDoughnutData = computed(() => ({
  labels:   statusItems.value.map(i => i.label),
  datasets: [{
    data:            statusItems.value.map(i => i.count),
    backgroundColor: statusItems.value.map(i => i.color),
    borderWidth:     0,
    hoverOffset:     4,
  }],
}))

// ── Shared doughnut options ──────────────────────────────────────────────────
const doughnutOptions = {
  ...baseChartOptions,
  cutout: '68%',
}

// ── Date helpers ─────────────────────────────────────────────────────────────
function last14Days() {
  const days = []
  for (let i = 13; i >= 0; i--) {
    const d = new Date()
    d.setDate(d.getDate() - i)
    days.push(d.toISOString().slice(0, 10))
  }
  return days
}

function fmtDay(iso) {
  const d = new Date(iso + 'T00:00:00')
  return d.toLocaleDateString('en', { month: 'short', day: 'numeric' })
}
</script>

<style scoped>
/* Hallmark · macrostructure: Bento-adjacent briefing · genre: modern-minimal (clean/formal)
 * theme: DevPulse locked system (design.md) — slate/maroon workspace (accent moved off the teleradiology-matched indigo per a later request)
 * body: Inter · outlier(mono): JetBrains Mono
 * motion: number-reveal count-up only · nav/footer: owned by App.vue (out of scope)
 * rounded-2xl bordered panels, no card shadow — see design.md § Macrostructure family
 */
.analytics-root {
  background: var(--dp-paper);
  font-family: var(--dp-font-body);
  color: var(--dp-ink);
}

.dv-mono {
  font-family: var(--dp-font-mono);
  font-variant-numeric: tabular-nums;
}
</style>
