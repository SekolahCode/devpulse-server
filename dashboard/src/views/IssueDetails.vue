<template>
  <div class="max-w-4xl mx-auto px-6 py-8">

    <!-- Breadcrumb -->
    <div class="flex items-center gap-2 text-sm text-gray-500 mb-8">
      <router-link to="/" class="hover:text-gray-300 transition-colors">Projects</router-link>
      <span class="text-gray-700">/</span>
      <button @click="$router.back()" class="hover:text-gray-300 transition-colors">Issues</button>
      <span class="text-gray-700">/</span>
      <span class="text-gray-400">Detail</span>
    </div>

    <!-- Loading skeleton -->
    <div v-if="loading" class="space-y-4">
      <div class="h-8 w-2/3 bg-[#111119] rounded-lg animate-pulse" />
      <div class="h-4 w-1/3 bg-[#111119] rounded-lg animate-pulse" />
      <div class="grid grid-cols-4 gap-4 mt-6">
        <div v-for="i in 4" :key="i" class="h-20 bg-[#111119] rounded-xl animate-pulse" />
      </div>
    </div>

    <template v-else-if="issue">

      <!-- Title -->
      <div class="flex items-start gap-3 mb-6">
        <span :class="levelBadge(issue.level)"
              class="text-[10px] font-bold px-1.5 py-0.5 rounded-md uppercase tracking-wide shrink-0 mt-1">
          {{ issue.level }}
        </span>
        <h1 class="text-xl font-semibold text-white leading-snug wrap-break-word">{{ issue.title }}</h1>
      </div>

      <!-- Stats grid -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-6">
        <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
          <p class="text-[11px] text-gray-500 uppercase tracking-wide font-medium">Status</p>
          <p :class="statusColor(issue.status)" class="text-base font-semibold mt-1 capitalize">{{ issue.status }}</p>
        </div>
        <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
          <p class="text-[11px] text-gray-500 uppercase tracking-wide font-medium">Events</p>
          <p class="text-base font-semibold text-white mt-1">{{ issue.event_count?.toLocaleString() ?? '—' }}</p>
        </div>
        <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
          <p class="text-[11px] text-gray-500 uppercase tracking-wide font-medium">Last seen</p>
          <p class="text-base font-semibold text-white mt-1">{{ timeAgo(issue.last_seen) }}</p>
        </div>
        <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
          <p class="text-[11px] text-gray-500 uppercase tracking-wide font-medium">First seen</p>
          <p class="text-base font-semibold text-white mt-1">{{ timeAgo(issue.first_seen) }}</p>
        </div>
      </div>

      <!-- Actions -->
      <div class="flex gap-3 mb-8">
        <button @click="resolve"
          class="flex items-center gap-2 bg-emerald-600 hover:bg-emerald-500 text-white
                 px-4 py-2 rounded-lg text-sm font-medium transition-colors">
          ✓ Resolve
        </button>
        <button @click="ignore"
          class="flex items-center gap-2 bg-white/5 hover:bg-white/10 text-gray-300
                 px-4 py-2 rounded-lg text-sm transition-colors">
          Ignore
        </button>
      </div>

      <!-- Recent occurrences -->
      <div v-if="issue.events?.length" class="mb-6">
        <h2 class="text-[11px] text-gray-500 uppercase tracking-wide font-medium mb-3">
          Recent occurrences
        </h2>
        <div class="space-y-3">
          <div v-for="(event, idx) in issue.events" :key="event.id"
               class="bg-[#111119] border border-white/6 rounded-xl overflow-hidden">

            <!-- Event header -->
            <div class="flex items-center justify-between px-4 py-2.5 border-b border-white/5 bg-[#0d0d16]">
              <div class="flex items-center gap-2">
                <span class="text-[11px] text-gray-500 font-medium">
                  {{ idx === 0 ? 'Latest' : `Occurrence #${idx + 1}` }}
                </span>
                <span v-if="event.environment"
                      class="text-[10px] px-1.5 py-0.5 rounded font-bold uppercase tracking-wide"
                      :class="envBadge(event.environment)">
                  {{ event.environment }}
                </span>
              </div>
              <span class="text-[11px] text-gray-600 tabular-nums">
                {{ formatDate(event.created_at) }}
              </span>
            </div>

            <!-- Stack trace -->
            <div v-if="event.payload?.exception?.stacktrace?.length" class="px-4 py-3">
              <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-2">Stack trace</p>
              <div class="font-mono text-[12px] leading-5 overflow-x-auto">
                <div v-for="(frame, fi) in event.payload.exception.stacktrace"
                     :key="fi"
                     class="flex items-start gap-3 px-2 py-0.5 rounded hover:bg-white/3 transition-colors"
                     :class="fi === 0 ? 'text-gray-200' : 'text-gray-500'">
                  <span class="shrink-0 text-gray-700 w-5 text-right select-none tabular-nums text-[11px] pt-px">
                    {{ fi + 1 }}
                  </span>
                  <span>
                    <span v-if="frame.function" class="text-violet-400">{{ frame.function }}</span>
                    <span v-if="frame.function && frame.file" class="text-gray-600"> @ </span>
                    <span v-if="frame.file" class="text-cyan-500/80">{{ frame.file }}</span>
                    <span v-if="frame.line" class="text-gray-600">:{{ frame.line }}</span>
                    <span v-if="!frame.function && !frame.file" class="text-gray-700 italic">unknown frame</span>
                  </span>
                </div>
              </div>
            </div>

            <!-- Collapsible raw details -->
            <div :class="event.payload?.exception?.stacktrace?.length ? 'border-t border-white/5' : ''">
              <details class="group">
                <summary class="flex items-center justify-between px-4 py-2.5 text-[11px] text-gray-600
                               hover:text-gray-400 cursor-pointer select-none list-none transition-colors">
                  <span>Raw payload / context</span>
                  <span class="transition-transform group-open:rotate-180 inline-block">▾</span>
                </summary>
                <div class="px-4 pb-4 space-y-3">
                  <template v-if="event.payload?.user">
                    <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium">User</p>
                    <pre class="text-[11px] text-gray-400 bg-[#0a0a10] rounded-lg p-3 overflow-x-auto">{{ fmt(event.payload.user) }}</pre>
                  </template>
                  <template v-if="event.payload?.request">
                    <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium">Request</p>
                    <pre class="text-[11px] text-gray-400 bg-[#0a0a10] rounded-lg p-3 overflow-x-auto">{{ fmt(event.payload.request) }}</pre>
                  </template>
                  <template v-if="event.context">
                    <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium">Extra context</p>
                    <pre class="text-[11px] text-gray-400 bg-[#0a0a10] rounded-lg p-3 overflow-x-auto">{{ fmt(event.context) }}</pre>
                  </template>
                  <template v-if="!event.payload?.user && !event.payload?.request && !event.context">
                    <p class="text-[11px] text-gray-600 italic">No additional context captured</p>
                  </template>
                </div>
              </details>
            </div>

          </div>
        </div>
      </div>

      <!-- Fingerprint -->
      <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
        <p class="text-[11px] text-gray-500 uppercase tracking-wide font-medium mb-2">Fingerprint</p>
        <code class="text-[13px] text-gray-300 font-mono break-all">{{ issue.fingerprint }}</code>
      </div>

    </template>

    <!-- Not found -->
    <div v-else class="flex flex-col items-center justify-center py-24 text-center">
      <div class="text-3xl mb-3">🔍</div>
      <p class="text-gray-300 font-medium">Issue not found</p>
    </div>

  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import axios from 'axios'

const route   = useRoute()
const router  = useRouter()
const issue   = ref(null)
const loading = ref(true)

onMounted(async () => {
  try {
    const { data } = await axios.get(`/api/issues/${route.params.id}`)
    issue.value = data
  } finally {
    loading.value = false
  }
})

async function resolve() {
  await axios.patch(`/api/issues/${route.params.id}`, { status: 'resolved' })
  router.back()
}

async function ignore() {
  await axios.patch(`/api/issues/${route.params.id}`, { status: 'ignored' })
  router.back()
}

const levelBadge = (level) =>
  ({ error: 'bg-red-500/15 text-red-400', warning: 'bg-amber-500/15 text-amber-400', info: 'bg-blue-500/15 text-blue-400' })[level]
  ?? 'bg-gray-700/50 text-gray-400'

const statusColor = (s) =>
  ({ unresolved: 'text-red-400', resolved: 'text-emerald-400', ignored: 'text-gray-400' })[s] ?? 'text-gray-300'

const envBadge = (env) =>
  ({ production: 'bg-red-500/15 text-red-400', staging: 'bg-amber-500/15 text-amber-400',
     development: 'bg-emerald-500/15 text-emerald-400' })[env?.toLowerCase()]
  ?? 'bg-gray-700/50 text-gray-400'

function timeAgo(date) {
  if (!date) return '—'
  const diff = Math.floor((Date.now() - new Date(date)) / 1000)
  if (diff < 60)    return `${diff}s ago`
  if (diff < 3600)  return `${Math.floor(diff / 60)}m ago`
  if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`
  return `${Math.floor(diff / 86400)}d ago`
}

function formatDate(date) {
  if (!date) return '—'
  return new Date(date).toLocaleString(undefined, {
    month: 'short', day: 'numeric',
    hour: '2-digit', minute: '2-digit', second: '2-digit',
  })
}

function fmt(obj) {
  try {
    return JSON.stringify(obj, null, 2)
  } catch {
    return String(obj)
  }
}
</script>
