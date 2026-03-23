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
      <div class="grid grid-cols-2 md:grid-cols-5 gap-3 mb-6">
        <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
          <p class="text-[11px] text-gray-500 uppercase tracking-wide font-medium">Status</p>
          <p :class="statusColor(issue.status)" class="text-base font-semibold mt-1 capitalize">{{ issue.status }}</p>
        </div>
        <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
          <p class="text-[11px] text-gray-500 uppercase tracking-wide font-medium">Events</p>
          <p class="text-base font-semibold text-white mt-1">{{ issue.event_count?.toLocaleString() ?? '—' }}</p>
        </div>
        <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
          <p class="text-[11px] text-gray-500 uppercase tracking-wide font-medium">Affected users</p>
          <p class="text-base font-semibold text-amber-400 mt-1">{{ issue.affected_users?.toLocaleString() ?? '0' }}</p>
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

      <!-- Metadata row: assignee, priority, release -->
      <div class="flex flex-wrap items-center gap-3 mb-5">
        <!-- Priority badge -->
        <span :class="priorityBadge(issue.priority)"
              class="text-[10px] font-bold px-2 py-0.5 rounded-md uppercase tracking-wide">
          {{ issue.priority ?? 'medium' }} priority
        </span>
        <!-- Release -->
        <span v-if="issue.last_release" class="text-[11px] text-gray-400 bg-white/5 px-2 py-0.5 rounded font-mono">
          v{{ issue.last_release }}
        </span>
        <!-- Assignee badge / set assignee -->
        <div class="flex items-center gap-1.5">
          <span v-if="issue.assignee" class="text-[11px] text-sky-400 bg-sky-500/10 px-2 py-0.5 rounded">
            👤 {{ issue.assignee }}
          </span>
          <button v-if="!showAssignee" @click="showAssignee = true"
            class="text-[11px] text-gray-600 hover:text-gray-400 transition-colors">
            {{ issue.assignee ? 'Reassign' : '+ Assign' }}
          </button>
          <form v-else @submit.prevent="setAssignee" class="flex gap-1">
            <input v-model="assigneeInput" placeholder="username" autofocus
              class="text-[11px] bg-[#111119] border border-white/10 rounded px-2 py-0.5 text-white w-28 focus:outline-none focus:border-violet-500" />
            <button type="submit" class="text-[11px] text-violet-400 hover:text-violet-300 px-1">✓</button>
            <button type="button" @click="showAssignee = false" class="text-[11px] text-gray-600 hover:text-gray-400 px-1">✕</button>
          </form>
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
              <div class="flex items-center gap-2">
                <span v-if="event.release"
                      class="text-[10px] font-mono text-gray-500 bg-white/5 px-1.5 py-0.5 rounded">
                  v{{ event.release }}
                </span>
                <span class="text-[11px] text-gray-600 tabular-nums">
                  {{ formatDate(event.created_at) }}
                </span>
              </div>
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

            <!-- Breadcrumbs -->
            <div v-if="event.breadcrumbs?.length" class="px-4 py-3 border-t border-white/5">
              <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-2">Breadcrumbs</p>
              <div class="space-y-1 font-mono text-[11px]">
                <div v-for="(crumb, ci) in event.breadcrumbs" :key="ci"
                     class="flex items-start gap-2 text-gray-500">
                  <span class="shrink-0 text-gray-700 tabular-nums">{{ crumb.timestamp ? crumb.timestamp.replace('T', ' ').slice(0, 19) : '' }}</span>
                  <span v-if="crumb.category" :class="crumbColor(crumb.level)" class="shrink-0">{{ crumb.category }}</span>
                  <span class="text-gray-400 truncate">{{ crumb.message }}</span>
                </div>
              </div>
            </div>

            <!-- Fatal error notice -->
            <div v-if="event.payload?.is_fatal" class="px-4 py-3 border-t border-white/5">
              <div class="flex items-start gap-2 bg-amber-500/10 border border-amber-500/20 rounded-lg px-3 py-2.5">
                <span class="text-sm shrink-0 mt-px">⚠️</span>
                <div>
                  <p class="text-[11px] text-amber-300 font-semibold">
                    Fatal error{{ event.payload.error_type ? ` (${event.payload.error_type})` : '' }} — captured by shutdown handler
                  </p>
                  <p class="text-[11px] text-amber-500/80 mt-0.5">
                    PHP fatal errors terminate the script before the call stack is available.
                    Only the error origin file and line are shown above.
                  </p>
                </div>
              </div>
            </div>

            <!-- Laravel command context -->
            <div v-if="event.payload?.command" class="px-4 py-3 border-t border-white/5">
              <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-2">Artisan Command</p>
              <div class="flex flex-wrap items-center gap-2">
                <span class="text-[12px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-1 rounded">
                  php artisan {{ event.payload.command }}
                </span>
                <span v-if="event.payload.exit_code !== undefined"
                      class="text-[11px] font-mono text-red-400 bg-red-500/10 px-2 py-0.5 rounded">
                  exit {{ event.payload.exit_code }}
                </span>
                <span v-if="event.payload.input?.trim()"
                      class="text-[11px] text-gray-400 font-mono">
                  {{ event.payload.input }}
                </span>
              </div>
              <div v-if="event.payload.laravel || event.payload.php" class="flex gap-2 mt-2">
                <span v-if="event.payload.laravel"
                      class="text-[10px] text-purple-400 bg-purple-500/10 px-1.5 py-0.5 rounded font-mono">
                  Laravel {{ event.payload.laravel }}
                </span>
                <span v-if="event.payload.php"
                      class="text-[10px] text-blue-400 bg-blue-500/10 px-1.5 py-0.5 rounded font-mono">
                  PHP {{ event.payload.php }}
                </span>
              </div>
            </div>

            <!-- Request -->
            <div v-if="event.payload?.request" class="px-4 py-3 border-t border-white/5">
              <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-2">Request</p>
              <div class="flex flex-wrap items-center gap-2">
                <span :class="methodColor(event.payload.request.method)"
                      class="text-[11px] font-bold px-2 py-0.5 rounded font-mono">
                  {{ event.payload.request.method }}
                </span>
                <span class="text-[12px] text-gray-300 font-mono break-all leading-relaxed">{{ event.payload.request.url }}</span>
                <span v-if="event.payload.request.ip" class="text-[11px] text-gray-600 font-mono shrink-0">
                  {{ event.payload.request.ip }}
                </span>
              </div>
            </div>

            <!-- User -->
            <div v-if="event.payload?.user" class="px-4 py-3 border-t border-white/5">
              <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-2">User</p>
              <div class="flex flex-wrap gap-2">
                <span v-if="event.payload.user.id"       class="text-[11px] text-gray-400 bg-white/5 px-2 py-0.5 rounded font-mono">ID: {{ event.payload.user.id }}</span>
                <span v-if="event.payload.user.email"    class="text-[11px] text-gray-400 bg-white/5 px-2 py-0.5 rounded">{{ event.payload.user.email }}</span>
                <span v-if="event.payload.user.username" class="text-[11px] text-gray-400 bg-white/5 px-2 py-0.5 rounded">{{ event.payload.user.username }}</span>
              </div>
            </div>

            <!-- WordPress platform metadata -->
            <div v-if="event.context?.platform === 'wordpress'" class="px-4 py-3 border-t border-white/5">
              <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-2">Platform</p>
              <div class="flex flex-wrap items-center gap-2 mb-2">
                <span v-if="event.context.wordpress" class="text-[11px] font-semibold px-2 py-0.5 rounded bg-blue-600/20 text-blue-300">
                  WP {{ event.context.wordpress }}
                </span>
                <span v-if="event.context.php" class="text-[11px] font-semibold px-2 py-0.5 rounded bg-purple-600/20 text-purple-300">
                  PHP {{ event.context.php }}
                </span>
                <span v-if="event.context.theme" class="text-[11px] text-gray-400 bg-white/5 px-2 py-0.5 rounded">
                  🎨 {{ event.context.theme }}
                </span>
                <span v-if="event.context.memory" class="text-[11px] text-gray-500 bg-white/5 px-2 py-0.5 rounded font-mono">
                  {{ formatMemory(event.context.memory) }} peak mem
                </span>
              </div>
              <div class="flex flex-wrap gap-1.5">
                <span v-if="event.context.is_admin"    class="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/15 text-amber-400 font-semibold">admin</span>
                <span v-if="event.context.multisite"   class="text-[10px] px-1.5 py-0.5 rounded bg-blue-500/15 text-blue-400 font-semibold">multisite</span>
                <span v-if="event.context.wp_debug"    class="text-[10px] px-1.5 py-0.5 rounded bg-orange-500/15 text-orange-400 font-semibold">WP_DEBUG</span>
                <span v-if="event.context.wp_debug_log" class="text-[10px] px-1.5 py-0.5 rounded bg-orange-500/15 text-orange-400 font-semibold">DEBUG_LOG</span>
              </div>
            </div>

            <!-- Active plugins (collapsible) -->
            <div v-if="event.context?.active_plugins?.length" class="px-4 py-3 border-t border-white/5">
              <div class="flex items-center justify-between mb-1.5">
                <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium">Active plugins</p>
                <button @click="togglePlugins(event.id)"
                  class="text-[10px] text-gray-600 hover:text-gray-400 transition-colors tabular-nums">
                  {{ expandedPlugins.has(event.id)
                      ? '▲ hide'
                      : `▼ ${event.context.plugin_count ?? event.context.active_plugins.length} plugins` }}
                </button>
              </div>
              <div v-if="expandedPlugins.has(event.id)"
                   class="font-mono text-[11px] text-gray-500 space-y-0.5 max-h-36 overflow-y-auto
                          scrollbar-thin scrollbar-thumb-white/10">
                <div v-for="p in event.context.active_plugins" :key="p"
                     class="truncate hover:text-gray-400 transition-colors">{{ p }}</div>
              </div>
              <div v-else class="text-[11px] text-gray-700 italic">Click to expand</div>
            </div>

            <!-- Extra context: non-WP platforms, or raw fallback -->
            <div v-if="event.context && event.context.platform !== 'wordpress'" class="border-t border-white/5">
              <details class="group">
                <summary class="flex items-center justify-between px-4 py-2.5 text-[11px] text-gray-600
                               hover:text-gray-400 cursor-pointer select-none list-none transition-colors">
                  <span>Extra context</span>
                  <span class="transition-transform group-open:rotate-180 inline-block">▾</span>
                </summary>
                <div class="px-4 pb-4">
                  <pre class="text-[11px] text-gray-400 bg-[#0a0a10] rounded-lg p-3 overflow-x-auto">{{ fmt(event.context) }}</pre>
                </div>
              </details>
            </div>

            <!-- Empty state -->
            <div v-if="!event.payload?.is_fatal && !event.payload?.request && !event.payload?.user && !event.context"
                 class="px-4 py-3 border-t border-white/5">
              <p class="text-[11px] text-gray-600 italic">No additional context captured</p>
            </div>

          </div>
        </div>
      </div>

      <!-- AI Analysis panel -->
      <div class="mb-6">
        <div class="flex items-center justify-between mb-3">
          <h2 class="text-[11px] text-gray-500 uppercase tracking-wide font-medium flex items-center gap-1.5">
            <span>AI Analysis</span>
            <span v-if="ai?.cached" class="text-[9px] px-1.5 py-0.5 rounded bg-gray-700/60 text-gray-500 font-bold tracking-wider">cached</span>
          </h2>
          <button
            @click="runAnalysis"
            :disabled="aiLoading"
            class="flex items-center gap-1.5 text-[11px] px-3 py-1.5 rounded-lg font-medium transition-all
                   bg-violet-600/20 text-violet-400 hover:bg-violet-600/30 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <svg v-if="aiLoading" class="animate-spin w-3 h-3" viewBox="0 0 24 24" fill="none">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8H4z"/>
            </svg>
            <svg v-else width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
            </svg>
            {{ aiLoading ? 'Analysing…' : ai ? 'Re-analyse' : 'Analyse with AI' }}
          </button>
        </div>

        <!-- Error -->
        <div v-if="aiError" class="bg-red-500/10 border border-red-500/20 rounded-xl px-4 py-3 text-sm text-red-400">
          {{ aiError }}
        </div>

        <!-- Results -->
        <div v-else-if="ai" class="space-y-3">
          <!-- Severity + root cause row -->
          <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
            <div class="flex items-start gap-3">
              <span :class="severityBadge(ai.severity)"
                    class="shrink-0 text-[10px] font-bold px-2 py-0.5 rounded-md uppercase tracking-wider mt-0.5">
                {{ ai.severity }}
              </span>
              <p class="text-[13px] text-gray-200 font-medium leading-snug">{{ ai.root_cause }}</p>
            </div>
          </div>

          <!-- Explanation -->
          <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
            <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-1.5">Explanation</p>
            <p class="text-[13px] text-gray-300 leading-relaxed">{{ ai.explanation }}</p>
          </div>

          <!-- Fix suggestion -->
          <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
            <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-1.5">How to fix</p>
            <p class="text-[13px] text-gray-300 leading-relaxed">{{ ai.fix_suggestion }}</p>
          </div>

          <!-- Code example -->
          <div v-if="ai.code_example" class="bg-[#111119] border border-white/6 rounded-xl overflow-hidden">
            <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium px-4 pt-3 pb-2">Code example</p>
            <pre class="text-[12px] text-gray-300 font-mono bg-[#0a0a10] px-4 pb-4 overflow-x-auto leading-5 whitespace-pre-wrap">{{ ai.code_example }}</pre>
          </div>

          <!-- Prevention -->
          <div v-if="ai.prevention" class="bg-[#111119] border border-white/6 rounded-xl p-4">
            <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-1.5">Prevention</p>
            <p class="text-[13px] text-gray-400 leading-relaxed">{{ ai.prevention }}</p>
          </div>

          <p class="text-[10px] text-gray-700 text-right">
            Powered by {{ ai.model }} · {{ ai.cached ? 'cached result' : 'fresh analysis' }}
          </p>
        </div>

        <!-- Prompt -->
        <div v-else class="bg-[#111119] border border-white/6 rounded-xl px-4 py-6 text-center">
          <p class="text-[13px] text-gray-500">Click <span class="text-violet-400 font-medium">Analyse with AI</span> to get root cause, fix suggestions, and prevention tips powered by Claude.</p>
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

const route           = useRoute()
const router          = useRouter()
const issue           = ref(null)
const loading         = ref(true)
const ai              = ref(null)
const aiLoading       = ref(false)
const aiError         = ref(null)
const showAssignee    = ref(false)
const assigneeInput   = ref('')
const expandedPlugins = ref(new Set())

function togglePlugins(eventId) {
  const s = new Set(expandedPlugins.value)
  if (s.has(eventId)) s.delete(eventId)
  else s.add(eventId)
  expandedPlugins.value = s
}

onMounted(async () => {
  try {
    const [issueRes] = await Promise.all([
      axios.get(`/api/issues/${route.params.id}`),
      // Silently load cached AI analysis if it exists
      axios.get(`/api/issues/${route.params.id}/analyze`)
        .then(r => { ai.value = r.data })
        .catch(() => {}), // 404 means no analysis yet — that's fine
    ])
    issue.value = issueRes.data
  } finally {
    loading.value = false
  }
})

async function runAnalysis() {
  aiLoading.value = true
  aiError.value   = null
  try {
    const { data } = await axios.post(`/api/issues/${route.params.id}/analyze`)
    ai.value = data
  } catch (err) {
    if (err.response?.status === 429) {
      aiError.value = 'Analysis was run recently. Please wait a moment before re-analysing.'
    } else {
      aiError.value = err.response?.data?.error ?? 'Analysis failed. Check that ANTHROPIC_API_KEY is configured.'
    }
  } finally {
    aiLoading.value = false
  }
}

async function resolve() {
  await axios.patch(`/api/issues/${route.params.id}`, { status: 'resolved' })
  router.back()
}

async function ignore() {
  await axios.patch(`/api/issues/${route.params.id}`, { status: 'ignored' })
  router.back()
}

async function setAssignee() {
  const val = assigneeInput.value.trim()
  await axios.patch(`/api/issues/${route.params.id}`, { assignee: val || '__clear__' })
  issue.value = { ...issue.value, assignee: val || null }
  showAssignee.value = false
  assigneeInput.value = ''
}

const methodColor = (m) =>
  ({ GET: 'bg-emerald-500/20 text-emerald-400', POST: 'bg-blue-500/20 text-blue-400',
     PUT: 'bg-amber-500/20 text-amber-400', PATCH: 'bg-amber-500/20 text-amber-400',
     DELETE: 'bg-red-500/20 text-red-400' })[m] ?? 'bg-gray-700/50 text-gray-400'

function formatMemory(bytes) {
  if (!bytes) return ''
  if (bytes >= 1073741824) return (bytes / 1073741824).toFixed(1) + ' GB'
  if (bytes >= 1048576)    return (bytes / 1048576).toFixed(0) + ' MB'
  return (bytes / 1024).toFixed(0) + ' KB'
}

const priorityBadge = (p) =>
  ({ critical: 'bg-red-500/20 text-red-400', high: 'bg-orange-500/20 text-orange-400',
     medium: 'bg-amber-500/20 text-amber-400', low: 'bg-blue-500/20 text-blue-400' })[p]
  ?? 'bg-gray-700/50 text-gray-400'

const crumbColor = (level) =>
  ({ error: 'text-red-400', warning: 'text-amber-400', info: 'text-blue-400', debug: 'text-gray-600' })[level]
  ?? 'text-gray-600'

const severityBadge = (s) =>
  ({ critical: 'bg-red-500/20 text-red-400', high: 'bg-orange-500/20 text-orange-400',
     medium: 'bg-amber-500/20 text-amber-400', low: 'bg-blue-500/20 text-blue-400' })[s]
  ?? 'bg-gray-700/50 text-gray-400'

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
