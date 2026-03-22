<template>
  <div class="max-w-4xl mx-auto px-6 py-8">

    <!-- Header -->
    <div class="flex items-center justify-between mb-8">
      <div class="flex items-center gap-3">
        <router-link to="/" class="text-gray-500 hover:text-gray-300 text-sm transition-colors">Projects</router-link>
        <span class="text-gray-700">/</span>
        <router-link :to="`/projects/${route.params.id}/issues`" class="text-gray-500 hover:text-gray-300 text-sm transition-colors">Issues</router-link>
        <span class="text-gray-700">/</span>
        <span class="text-[15px] font-semibold text-white">Releases</span>
      </div>
      <button
        @click="showCreate = true"
        class="flex items-center gap-2 bg-violet-600 hover:bg-violet-500 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors shadow-lg shadow-violet-900/30"
      >
        <span class="text-base leading-none">+</span>
        New Release
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="space-y-3">
      <div v-for="i in 4" :key="i" class="h-20 bg-[#111119] rounded-xl animate-pulse" />
    </div>

    <!-- Empty -->
    <div v-else-if="!releases.length" class="flex flex-col items-center justify-center py-24 text-center">
      <div class="w-12 h-12 rounded-full bg-violet-500/10 flex items-center justify-center text-2xl mb-4">🏷️</div>
      <p class="text-gray-300 font-medium">No releases yet</p>
      <p class="text-gray-500 text-sm mt-1">Track deployments and correlate them with issues</p>
    </div>

    <!-- Timeline -->
    <div v-else class="relative">
      <!-- Timeline line -->
      <div class="absolute left-[19px] top-5 bottom-5 w-px bg-white/5 pointer-events-none" />

      <div class="space-y-4">
        <div
          v-for="(rel, idx) in releases"
          :key="rel.id"
          class="relative flex gap-4 group"
        >
          <!-- Timeline dot -->
          <div class="shrink-0 w-10 flex justify-center pt-4">
            <div :class="[
              'w-2.5 h-2.5 rounded-full border-2 transition-colors z-10',
              idx === 0
                ? 'bg-violet-500 border-violet-400 shadow-lg shadow-violet-500/40'
                : 'bg-[#111119] border-white/20 group-hover:border-white/40'
            ]" />
          </div>

          <!-- Card -->
          <div class="flex-1 bg-[#111119] border border-white/6 rounded-xl p-4 hover:border-white/10 transition-all group-hover:bg-[#13131f] mb-1">
            <div class="flex items-start justify-between gap-3 mb-3">
              <!-- Version tag -->
              <div class="flex items-center gap-2.5">
                <span :class="versionBadge(rel.version)"
                      class="text-[11px] font-bold px-2 py-0.5 rounded-md font-mono tracking-wide">
                  v{{ rel.version }}
                </span>
                <span v-if="idx === 0" class="text-[9px] font-bold px-1.5 py-0.5 rounded bg-violet-500/20 text-violet-400 uppercase tracking-wider">
                  Latest
                </span>
              </div>
              <!-- Actions -->
              <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                <a
                  v-if="rel.url"
                  :href="rel.url"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="w-6 h-6 flex items-center justify-center rounded text-gray-600 hover:text-blue-400 hover:bg-blue-500/10 transition-colors"
                  title="Open deploy URL"
                >
                  <svg width="11" height="11" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M7 3H3v10h10V9M9 1h6v6M15 1l-8 8"/>
                  </svg>
                </a>
                <button
                  @click="confirmDelete(rel)"
                  class="w-6 h-6 flex items-center justify-center rounded text-gray-600 hover:text-red-400 hover:bg-red-500/10 transition-colors"
                  title="Delete release"
                >
                  <svg width="11" height="11" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                    <line x1="3" y1="3" x2="13" y2="13"/><line x1="13" y1="3" x2="3" y2="13"/>
                  </svg>
                </button>
              </div>
            </div>

            <!-- Meta: git ref + deploy date -->
            <div class="flex flex-wrap items-center gap-3 mb-3">
              <span v-if="rel.ref" class="flex items-center gap-1 text-[11px] text-gray-500 font-mono">
                <svg width="11" height="11" viewBox="0 0 16 16" fill="currentColor" class="text-gray-600">
                  <path d="M11.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zm-2.25.75a2.25 2.25 0 1 1 3 2.122V6A2.5 2.5 0 0 1 10 8.5H6a1 1 0 0 0-1 1v1.128a2.251 2.251 0 1 1-1.5 0V5.372a2.25 2.25 0 1 1 1.5 0v1.836A2.492 2.492 0 0 1 6 7h4a1 1 0 0 0 1-1v-.628A2.25 2.25 0 0 1 9.5 3.25zM4.25 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zM4.25 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5z"/>
                </svg>
                {{ rel.ref.slice(0, 8) }}
              </span>
              <span class="text-[11px] text-gray-600">
                {{ formatDate(rel.deployed_at) }}
              </span>
            </div>

            <!-- Issue stats -->
            <div class="flex items-center gap-4">
              <router-link
                v-if="rel.new_issues > 0"
                :to="`/projects/${route.params.id}/issues?release=${rel.version}`"
                class="flex items-center gap-1.5 text-[11px] text-red-400 hover:text-red-300 transition-colors"
                title="New issues introduced in this release"
              >
                <span class="w-1.5 h-1.5 rounded-full bg-red-400" />
                {{ rel.new_issues }} new {{ rel.new_issues === 1 ? 'issue' : 'issues' }}
              </router-link>
              <router-link
                v-if="rel.open_issues > 0"
                :to="`/projects/${route.params.id}/issues?release=${rel.version}`"
                class="flex items-center gap-1.5 text-[11px] text-amber-400 hover:text-amber-300 transition-colors"
                title="Issues still open in this release"
              >
                <span class="w-1.5 h-1.5 rounded-full bg-amber-400" />
                {{ rel.open_issues }} open
              </router-link>
              <span
                v-if="rel.resolved_issues > 0"
                class="flex items-center gap-1.5 text-[11px] text-emerald-500"
                title="Issues resolved by this release"
              >
                <span class="w-1.5 h-1.5 rounded-full bg-emerald-500" />
                {{ rel.resolved_issues }} resolved
              </span>
              <span v-if="!rel.new_issues && !rel.open_issues && !rel.resolved_issues"
                    class="text-[11px] text-gray-600 italic">
                No tracked issues
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- ─── Create Release Modal ──────────────────────────────────────────────── -->
    <Transition name="modal">
      <div
        v-if="showCreate"
        class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 px-4"
        @click.self="showCreate = false"
      >
        <div class="bg-[#13131c] border border-white/8 rounded-2xl p-6 w-full max-w-sm shadow-2xl">
          <h2 class="text-lg font-semibold text-white mb-5">New Release</h2>
          <div class="space-y-3">
            <div>
              <label class="block text-xs text-gray-400 mb-1.5 font-medium">Version <span class="text-red-400">*</span></label>
              <input
                v-model="form.version"
                placeholder="e.g. 1.4.2 or 2.0.0-beta.1"
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2.5 text-sm text-white
                       placeholder-gray-600 focus:outline-none focus:border-violet-500 transition-colors font-mono"
              />
            </div>
            <div>
              <label class="block text-xs text-gray-400 mb-1.5 font-medium">Git ref / commit SHA <span class="text-gray-600">(optional)</span></label>
              <input
                v-model="form.ref"
                placeholder="abc1234 or refs/tags/v1.4.2"
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2.5 text-sm text-white
                       placeholder-gray-600 focus:outline-none focus:border-violet-500 transition-colors font-mono"
              />
            </div>
            <div>
              <label class="block text-xs text-gray-400 mb-1.5 font-medium">Deploy URL <span class="text-gray-600">(optional)</span></label>
              <input
                v-model="form.url"
                placeholder="https://ci.example.com/builds/123"
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2.5 text-sm text-white
                       placeholder-gray-600 focus:outline-none focus:border-violet-500 transition-colors"
              />
            </div>
          </div>
          <p v-if="createError" class="text-xs text-red-400 mt-2">{{ createError }}</p>
          <div class="flex gap-3 mt-5">
            <button
              @click="createRelease"
              :disabled="!form.version.trim() || creating"
              class="flex-1 bg-violet-600 hover:bg-violet-500 disabled:opacity-40 disabled:cursor-not-allowed
                     text-white py-2.5 rounded-lg text-sm font-medium transition-colors"
            >
              {{ creating ? 'Creating…' : 'Create release' }}
            </button>
            <button
              @click="showCreate = false"
              class="flex-1 bg-white/5 hover:bg-white/10 text-gray-300 py-2.5 rounded-lg text-sm transition-colors"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- ─── Delete Confirm Modal ──────────────────────────────────────────────── -->
    <Transition name="modal">
      <div
        v-if="deleteTarget"
        class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 px-4"
        @click.self="deleteTarget = null"
      >
        <div class="bg-[#13131c] border border-white/8 rounded-2xl p-6 w-full max-w-sm shadow-2xl">
          <h2 class="text-lg font-semibold text-white mb-2">Delete release?</h2>
          <p class="text-sm text-gray-400 mb-6">
            Remove <span class="font-mono text-white">v{{ deleteTarget.version }}</span> from the timeline.
            Issue linkage will be preserved but the release entry will be gone.
          </p>
          <div class="flex gap-3">
            <button
              @click="doDelete"
              class="flex-1 bg-red-600 hover:bg-red-500 text-white py-2.5 rounded-lg text-sm font-medium transition-colors"
            >
              Delete
            </button>
            <button
              @click="deleteTarget = null"
              class="flex-1 bg-white/5 hover:bg-white/10 text-gray-300 py-2.5 rounded-lg text-sm transition-colors"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </Transition>

  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import axios from 'axios'

const route       = useRoute()
const releases    = ref([])
const loading     = ref(true)
const showCreate  = ref(false)
const creating    = ref(false)
const createError = ref('')
const deleteTarget = ref(null)

const form = ref({ version: '', ref: '', url: '' })

onMounted(async () => {
  await load()
})

async function load() {
  loading.value = true
  try {
    const { data } = await axios.get(`/api/projects/${route.params.id}/releases`)
    releases.value = data.data
  } finally {
    loading.value = false
  }
}

async function createRelease() {
  if (!form.value.version.trim()) return
  creating.value    = true
  createError.value = ''
  try {
    const body = { version: form.value.version.trim() }
    if (form.value.ref.trim())  body.ref = form.value.ref.trim()
    if (form.value.url.trim())  body.url = form.value.url.trim()
    const { data } = await axios.post(`/api/projects/${route.params.id}/releases`, body)
    releases.value.unshift({ ...data, new_issues: 0, open_issues: 0, resolved_issues: 0 })
    showCreate.value  = false
    form.value        = { version: '', ref: '', url: '' }
  } catch (err) {
    createError.value = err.response?.data?.error ?? 'Failed to create release'
  } finally {
    creating.value = false
  }
}

function confirmDelete(rel) { deleteTarget.value = rel }

async function doDelete() {
  await axios.delete(`/api/releases/${deleteTarget.value.id}`)
  releases.value = releases.value.filter(r => r.id !== deleteTarget.value.id)
  deleteTarget.value = null
}

// Semver-aware badge colors
// major bump (x.0.0) → red/orange, minor (x.y.0) → blue, patch (x.y.z) → green, pre-release → gray
function versionBadge(v) {
  if (!v) return 'bg-gray-700/50 text-gray-400'
  const pre = v.includes('-') || v.includes('+')
  if (pre) return 'bg-gray-700/50 text-gray-300 font-mono'
  const parts = v.split('.').map(Number)
  const [major, minor, patch] = parts
  if (isNaN(major)) return 'bg-gray-700/50 text-gray-400'
  if (minor === 0 && patch === 0) return 'bg-red-500/20 text-red-300 font-mono'      // x.0.0 — major
  if (patch === 0)                return 'bg-blue-500/20 text-blue-300 font-mono'     // x.y.0 — minor
  return 'bg-emerald-500/20 text-emerald-300 font-mono'                               // x.y.z — patch
}

function formatDate(date) {
  if (!date) return '—'
  return new Date(date).toLocaleDateString(undefined, {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: '2-digit', minute: '2-digit',
  })
}
</script>

<style scoped>
.modal-enter-active, .modal-leave-active { transition: opacity 0.15s ease; }
.modal-enter-from, .modal-leave-to { opacity: 0; }
</style>
