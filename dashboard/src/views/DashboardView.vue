<template>
  <div class="max-w-6xl mx-auto px-6 py-8 space-y-8">

    <!-- Page header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-semibold text-white tracking-tight">Analytics</h1>
        <p class="text-sm text-muted-foreground mt-0.5">Last 14 days across all projects</p>
      </div>

      <!-- Time range selector using shadcn Select -->
      <Select v-model="range">
        <SelectTrigger class="w-36 bg-card border-border text-sm">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="14">Last 14 days</SelectItem>
          <SelectItem value="7">Last 7 days</SelectItem>
          <SelectItem value="30">Last 30 days</SelectItem>
        </SelectContent>
      </Select>
    </div>

    <!-- Stat cards -->
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
      <template v-if="loading">
        <Skeleton v-for="i in 4" :key="i" class="h-28 rounded-xl" />
      </template>
      <template v-else>
        <Card class="px-6 py-5 gap-3">
          <CardContent class="p-0">
            <p class="text-xs text-muted-foreground uppercase tracking-wide font-medium">Unresolved</p>
            <p class="text-3xl font-bold text-red-400 mt-1 tabular-nums">
              {{ stats?.issues.unresolved ?? 0 }}
            </p>
            <p class="text-xs text-muted-foreground mt-1">open issues</p>
          </CardContent>
        </Card>

        <Card class="px-6 py-5 gap-3">
          <CardContent class="p-0">
            <p class="text-xs text-muted-foreground uppercase tracking-wide font-medium">New 24 h</p>
            <p class="text-3xl font-bold text-amber-400 mt-1 tabular-nums">
              {{ stats?.issues.new_24h ?? 0 }}
            </p>
            <p class="text-xs text-muted-foreground mt-1">new issues today</p>
          </CardContent>
        </Card>

        <Card class="px-6 py-5 gap-3">
          <CardContent class="p-0">
            <p class="text-xs text-muted-foreground uppercase tracking-wide font-medium">Regressions</p>
            <p class="text-3xl font-bold text-orange-400 mt-1 tabular-nums">
              {{ stats?.issues.regressions_24h ?? 0 }}
            </p>
            <p class="text-xs text-muted-foreground mt-1">in last 24 h</p>
          </CardContent>
        </Card>

        <Card class="px-6 py-5 gap-3">
          <CardContent class="p-0">
            <p class="text-xs text-muted-foreground uppercase tracking-wide font-medium">Events 24 h</p>
            <p class="text-3xl font-bold text-violet-400 mt-1 tabular-nums">
              {{ (stats?.events_24h ?? 0).toLocaleString() }}
            </p>
            <p class="text-xs text-muted-foreground mt-1">events ingested</p>
          </CardContent>
        </Card>
      </template>
    </div>

    <!-- Timeline charts row -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">

      <!-- Events + Issues over time — Line chart (2/3 width) -->
      <Card class="lg:col-span-2 px-6 py-5 gap-4">
        <CardHeader class="p-0">
          <CardTitle class="text-[15px] font-semibold text-foreground">Activity timeline</CardTitle>
          <CardDescription>Events ingested and new issues opened per day</CardDescription>
        </CardHeader>
        <CardContent class="p-0">
          <Skeleton v-if="loading" class="h-56 w-full rounded-lg" />
          <div v-else class="h-56">
            <Line :data="lineChartData" :options="lineChartOptions" />
          </div>
        </CardContent>
      </Card>

      <!-- Issues by level — Doughnut (1/3 width) -->
      <Card class="px-6 py-5 gap-4">
        <CardHeader class="p-0">
          <CardTitle class="text-[15px] font-semibold text-foreground">Issues by level</CardTitle>
          <CardDescription>Unresolved issues breakdown</CardDescription>
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
                  <span class="text-muted-foreground capitalize">{{ item.label }}</span>
                </div>
                <span class="font-semibold text-foreground tabular-nums">{{ item.count }}</span>
              </div>
            </div>
          </template>
        </CardContent>
      </Card>
    </div>

    <!-- Bottom row -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">

      <!-- Top projects — Bar chart (2/3) -->
      <Card class="lg:col-span-2 px-6 py-5 gap-4">
        <CardHeader class="p-0">
          <CardTitle class="text-[15px] font-semibold text-foreground">Top projects</CardTitle>
          <CardDescription>Event volume by project over the last 7 days</CardDescription>
        </CardHeader>
        <CardContent class="p-0">
          <Skeleton v-if="loading" class="h-48 w-full rounded-lg" />
          <div v-else-if="!chartData?.top_projects?.length"
            class="h-48 flex items-center justify-center text-sm text-muted-foreground">
            No event data yet
          </div>
          <div v-else class="h-48">
            <Bar :data="barChartData" :options="barChartOptions" />
          </div>
        </CardContent>
      </Card>

      <!-- Issues by status — Doughnut (1/3) -->
      <Card class="px-6 py-5 gap-4">
        <CardHeader class="p-0">
          <CardTitle class="text-[15px] font-semibold text-foreground">Issue status</CardTitle>
          <CardDescription>Distribution across all projects</CardDescription>
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
                  <span class="text-muted-foreground capitalize">{{ item.label }}</span>
                </div>
                <span class="font-semibold text-foreground tabular-nums">{{ item.count }}</span>
              </div>
            </div>
          </template>
        </CardContent>
      </Card>
    </div>

  </div>
</template>

<script setup>
import { computed, onMounted, ref } from 'vue'
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

// ── Data fetching ────────────────────────────────────────────────────────────
onMounted(async () => {
  const [statsRes, chartRes] = await Promise.all([
    axios.get('/api/stats').catch(() => null),
    axios.get('/api/stats/chart').catch(() => null),
  ])
  stats.value     = statsRes?.data ?? null
  chartData.value = chartRes?.data ?? null
  loading.value   = false
})

// ── Shared Chart.js theme ────────────────────────────────────────────────────
const VIOLET  = 'oklch(0.546 0.245 281)'
const RED     = 'oklch(0.640 0.210 25)'
const AMBER   = 'oklch(0.728 0.160 68)'
const EMERALD = 'oklch(0.696 0.170 162)'
const BLUE    = 'oklch(0.627 0.220 264)'
const GRAY    = 'oklch(0.512 0.010 278)'

// Chart.js doesn't understand oklch — use hex equivalents
const C = {
  violet:  '#7c3aed',
  violetA: 'rgba(124,58,237,0.15)',
  red:     '#f87171',
  redA:    'rgba(248,113,113,0.15)',
  amber:   '#fbbf24',
  emerald: '#34d399',
  blue:    '#60a5fa',
  gray:    '#6b7280',
  border:  'rgba(255,255,255,0.06)',
  text:    '#9ca3af',
}

const baseChartOptions = {
  responsive:          true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: '#1a1a28',
      borderColor:     C.border,
      borderWidth:     1,
      titleColor:      '#e8e8f0',
      bodyColor:       C.text,
      padding:         10,
      cornerRadius:    8,
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
        color:       C.text,
        boxWidth:    10,
        boxHeight:   10,
        borderRadius: 3,
        usePointStyle: false,
        font: { size: 11 },
        padding: 16,
      },
    },
  },
  scales: {
    x: {
      grid:  { color: C.border },
      ticks: { color: C.text, font: { size: 11 }, maxRotation: 0 },
    },
    y: {
      grid:  { color: C.border },
      ticks: { color: C.text, font: { size: 11 }, precision: 0 },
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
      hoverBackgroundColor: '#8b5cf6',
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
      ticks: { color: C.text, font: { size: 11 } },
    },
    y: {
      grid:  { color: C.border },
      ticks: { color: C.text, font: { size: 11 }, precision: 0 },
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
