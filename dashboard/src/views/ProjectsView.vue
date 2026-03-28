<template>
  <div class="max-w-6xl mx-auto px-6 py-8">

    <!-- Page header -->
    <div class="flex items-center justify-between mb-8">
      <div>
        <h1 class="text-2xl font-semibold text-white tracking-tight">Projects</h1>
        <p class="text-sm text-gray-500 mt-0.5">Select a project to view its issues</p>
      </div>
      <button
        @click="showCreate = true"
        class="flex items-center gap-2 bg-violet-600 hover:bg-violet-500 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors shadow-lg shadow-violet-900/30"
      >
        <span class="text-base leading-none">+</span>
        New Project
      </button>
    </div>

    <!-- Loading skeleton -->
    <div v-if="!projectStore.loaded" class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div v-for="i in 4" :key="i" class="h-28 bg-[#111119] rounded-xl animate-pulse" />
    </div>

    <!-- Empty state -->
    <div v-else-if="!projectStore.projects.length" class="flex flex-col items-center justify-center py-24 text-center">
      <div class="w-12 h-12 rounded-full bg-violet-500/10 flex items-center justify-center text-2xl mb-4">📦</div>
      <p class="text-gray-300 font-medium">No projects yet</p>
      <p class="text-gray-500 text-sm mt-1">Create your first project to start tracking errors</p>
    </div>

    <!-- Grid -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div
        v-for="p in projectStore.projects"
        :key="p.id"
        class="group block bg-[#111119] border border-white/6 rounded-xl p-5 hover:border-violet-500/30 hover:bg-[#14141e] transition-all"
      >
        <div class="flex items-start justify-between gap-3">
          <router-link :to="`/projects/${p.id}/issues`" class="flex items-center gap-3 min-w-0 flex-1">
            <div :class="platformColor(p.platform)" class="w-9 h-9 rounded-lg flex items-center justify-center text-lg shrink-0">
              {{ platformIcon(p.platform) }}
            </div>
            <div class="min-w-0">
              <h2 class="font-semibold text-white text-[15px] truncate group-hover:text-violet-300 transition-colors">{{ p.name }}</h2>
              <p class="text-xs text-gray-500 mt-0.5 capitalize">{{ p.platform }}</p>
            </div>
          </router-link>

          <!-- Action buttons -->
          <div class="flex items-center gap-1 shrink-0">
            <button @click.prevent="openAlerts(p)" title="Manage alerts"
              class="w-7 h-7 flex items-center justify-center rounded text-gray-600 hover:text-amber-400 hover:bg-amber-500/10 transition-colors">
              <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                <path d="M8 1a5.5 5.5 0 0 0-5.5 5.5v2.293l-1.146 1.146A.5.5 0 0 0 1.5 11h13a.5.5 0 0 0 .354-.854L13.5 8.793V6.5A5.5 5.5 0 0 0 8 1zM6.5 13a1.5 1.5 0 0 0 3 0H6.5z"/>
              </svg>
            </button>
            <button @click.prevent="rotateKey(p)" title="Rotate API key"
              class="w-7 h-7 flex items-center justify-center rounded text-gray-600 hover:text-cyan-400 hover:bg-cyan-500/10 transition-colors">
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/>
              </svg>
            </button>
            <button @click.prevent="startEdit(p)" title="Edit project"
              class="w-7 h-7 flex items-center justify-center rounded text-gray-600 hover:text-gray-300 hover:bg-white/8 transition-colors">
              <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <path d="M11.5 2.5l2 2L5 13H3v-2L11.5 2.5z"/>
              </svg>
            </button>
            <button @click.prevent="confirmDelete(p)" title="Delete project"
              class="w-7 h-7 flex items-center justify-center rounded text-gray-600 hover:text-red-400 hover:bg-red-500/10 transition-colors">
              <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <path d="M2 4h12M5 4V2h6v2M6 7v5M10 7v5M3 4l1 9h8l1-9"/>
              </svg>
            </button>
          </div>
        </div>

        <!-- DSN -->
        <div class="mt-4 flex items-center gap-2 bg-black/20 rounded-lg px-3 py-2">
          <span class="text-[10px] text-gray-500 uppercase tracking-wide font-medium shrink-0">DSN</span>
          <code class="text-[11px] text-gray-400 truncate font-mono">{{ dsn(p.api_key) }}</code>
          <button @click.prevent="copy(p.api_key)"
            class="ml-auto text-gray-600 hover:text-gray-300 text-xs shrink-0 transition-colors" title="Copy DSN">⎘</button>
        </div>
      </div>
    </div>

    <!-- ─── Create Project Modal ──────────────────────────────────────────── -->
    <Transition name="modal">
      <div v-if="showCreate"
        class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 px-4"
        @click.self="showCreate = false">
        <div class="bg-[#13131c] border border-white/8 rounded-2xl p-6 w-full max-w-sm shadow-2xl">
          <h2 class="text-lg font-semibold text-white mb-5">New Project</h2>
          <div class="space-y-3">
            <div>
              <label class="block text-xs text-gray-400 mb-1.5 font-medium">Project name</label>
              <input v-model="createForm.name" placeholder="e.g. My Laravel App" autofocus
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2.5 text-sm text-white placeholder-gray-600 focus:outline-none focus:border-violet-500 transition-colors" />
            </div>
            <div>
              <label class="block text-xs text-gray-400 mb-1.5 font-medium">Platform</label>
              <select v-model="createForm.platform"
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2.5 text-sm text-white focus:outline-none focus:border-violet-500 transition-colors">
                <option v-for="p in PLATFORMS" :key="p" :value="p" class="capitalize">{{ p }}</option>
              </select>
            </div>
          </div>
          <p v-if="createError" class="text-[12px] text-red-400 mt-3">{{ createError }}</p>
          <div class="flex gap-3 mt-6">
            <button @click="create" :disabled="!createForm.name.trim()"
              class="flex-1 bg-violet-600 hover:bg-violet-500 disabled:opacity-40 disabled:cursor-not-allowed text-white py-2.5 rounded-lg text-sm font-medium transition-colors">
              Create project
            </button>
            <button @click="showCreate = false"
              class="flex-1 bg-white/5 hover:bg-white/10 text-gray-300 py-2.5 rounded-lg text-sm transition-colors">
              Cancel
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- ─── Edit Project Modal ────────────────────────────────────────────── -->
    <Transition name="modal">
      <div v-if="editProject"
        class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 px-4"
        @click.self="editProject = null">
        <div class="bg-[#13131c] border border-white/8 rounded-2xl p-6 w-full max-w-sm shadow-2xl">
          <h2 class="text-lg font-semibold text-white mb-5">Edit Project</h2>
          <div class="space-y-3">
            <div>
              <label class="block text-xs text-gray-400 mb-1.5 font-medium">Project name</label>
              <input v-model="editForm.name"
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2.5 text-sm text-white focus:outline-none focus:border-violet-500 transition-colors" />
            </div>
            <div>
              <label class="block text-xs text-gray-400 mb-1.5 font-medium">Platform</label>
              <select v-model="editForm.platform"
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2.5 text-sm text-white focus:outline-none focus:border-violet-500 transition-colors">
                <option v-for="p in PLATFORMS" :key="p" :value="p" class="capitalize">{{ p }}</option>
              </select>
            </div>
          </div>
          <div class="flex gap-3 mt-6">
            <button @click="saveEdit" :disabled="!editForm.name.trim()"
              class="flex-1 bg-violet-600 hover:bg-violet-500 disabled:opacity-40 disabled:cursor-not-allowed text-white py-2.5 rounded-lg text-sm font-medium transition-colors">
              Save changes
            </button>
            <button @click="editProject = null"
              class="flex-1 bg-white/5 hover:bg-white/10 text-gray-300 py-2.5 rounded-lg text-sm transition-colors">
              Cancel
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- ─── Delete Confirm Modal ──────────────────────────────────────────── -->
    <Transition name="modal">
      <div v-if="deleteTarget"
        class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 px-4"
        @click.self="deleteTarget = null">
        <div class="bg-[#13131c] border border-white/8 rounded-2xl p-6 w-full max-w-sm shadow-2xl">
          <h2 class="text-lg font-semibold text-white mb-2">Delete project?</h2>
          <p class="text-sm text-gray-400 mb-6">
            This will permanently delete <span class="text-white font-medium">{{ deleteTarget.name }}</span>
            and all its issues and events. This cannot be undone.
          </p>
          <div class="flex gap-3">
            <button @click="doDelete"
              class="flex-1 bg-red-600 hover:bg-red-500 text-white py-2.5 rounded-lg text-sm font-medium transition-colors">
              Delete permanently
            </button>
            <button @click="deleteTarget = null"
              class="flex-1 bg-white/5 hover:bg-white/10 text-gray-300 py-2.5 rounded-lg text-sm transition-colors">
              Cancel
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- ─── Alerts Modal ──────────────────────────────────────────────────── -->
    <Transition name="modal">
      <div v-if="alertsProject"
        class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 px-4"
        @click.self="alertsProject = null">
        <div class="bg-[#13131c] border border-white/8 rounded-2xl p-6 w-full max-w-lg shadow-2xl">
          <div class="flex items-center justify-between mb-5">
            <h2 class="text-lg font-semibold text-white">
              Alerts — <span class="text-violet-400">{{ alertsProject.name }}</span>
            </h2>
            <button @click="alertsProject = null" class="text-gray-600 hover:text-gray-300 text-lg leading-none">✕</button>
          </div>

          <div v-if="alerts.length" class="space-y-2 mb-4">
            <div v-for="a in alerts" :key="a.id"
              class="flex items-center gap-3 bg-[#0d0d16] border border-white/5 rounded-lg px-3 py-2.5">
              <span :class="channelIcon(a.channel).color" class="text-lg shrink-0">{{ channelIcon(a.channel).icon }}</span>
              <div class="min-w-0 flex-1">
                <p class="text-[12px] text-gray-300 truncate font-mono">{{ a.endpoint }}</p>
                <p class="text-[10px] text-gray-600 capitalize">{{ a.channel }} · {{ a.cooldown_minutes ?? 60 }}m cooldown</p>
              </div>
              <button @click="toggleAlert(a)"
                :class="a.enabled ? 'bg-emerald-500/20 text-emerald-400' : 'bg-gray-700/50 text-gray-500'"
                class="text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wide transition-colors">
                {{ a.enabled ? 'ON' : 'OFF' }}
              </button>
              <button @click="deleteAlertItem(a)"
                class="w-6 h-6 flex items-center justify-center rounded text-gray-600 hover:text-red-400 hover:bg-red-500/10 transition-colors shrink-0">
                <svg width="11" height="11" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                  <line x1="3" y1="3" x2="13" y2="13"/><line x1="13" y1="3" x2="3" y2="13"/>
                </svg>
              </button>
            </div>
          </div>
          <p v-else class="text-sm text-gray-500 mb-4">No alerts configured for this project.</p>

          <div class="border-t border-white/5 pt-4">
            <p class="text-[11px] text-gray-500 uppercase tracking-wide font-medium mb-3">Add Alert</p>
            <div class="space-y-2">
              <select v-model="alertForm.channel"
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-violet-500 transition-colors">
                <option v-for="ch in CHANNELS" :key="ch" :value="ch">{{ channelLabel(ch) }}</option>
              </select>
              <input v-model="alertForm.endpoint" :placeholder="alertPlaceholder"
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2 text-sm text-white placeholder-gray-600 focus:outline-none focus:border-violet-500 transition-colors font-mono" />
              <button @click="addAlert" :disabled="!alertForm.endpoint.trim()"
                class="w-full bg-violet-600 hover:bg-violet-500 disabled:opacity-40 disabled:cursor-not-allowed text-white py-2 rounded-lg text-sm font-medium transition-colors">
                Add alert
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>

  </div>
</template>

<script setup>
import axios from 'axios'
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useProjectStore } from '../stores/project'
import { useToastStore }   from '../stores/toast'
import { PLATFORMS, CHANNELS, platformIcon, platformColor } from '../composables/useColors'

const projectStore = useProjectStore()
const toast        = useToastStore()

const showCreate   = ref(false)
const editProject  = ref(null)
const deleteTarget = ref(null)
const alertsProject = ref(null)
const alerts        = ref([])
const createError   = ref('')

const createForm = ref({ name: '', platform: 'laravel' })
const editForm   = ref({ name: '', platform: 'laravel' })
const alertForm  = ref({ channel: 'webhook', endpoint: '' })

// ── Keyboard: Esc closes any open modal ──────────────────────────────────────
function onKeydown(e) {
  if (e.key !== 'Escape') return
  if (alertsProject.value) { alertsProject.value = null; return }
  if (deleteTarget.value)  { deleteTarget.value  = null; return }
  if (editProject.value)   { editProject.value   = null; return }
  if (showCreate.value)    { showCreate.value    = false }
}

onMounted(async () => {
  document.addEventListener('keydown', onKeydown)
  try { await projectStore.load() }
  catch { toast.error('Failed to load projects') }
})
onUnmounted(() => document.removeEventListener('keydown', onKeydown))

// ── Create ────────────────────────────────────────────────────────────────────
async function create() {
  if (!createForm.value.name.trim()) return
  createError.value = ''
  try {
    await projectStore.create(createForm.value)
    showCreate.value = false
    createForm.value = { name: '', platform: 'laravel' }
  } catch {
    createError.value = 'Failed to create project. Please try again.'
  }
}

// ── Edit ──────────────────────────────────────────────────────────────────────
function startEdit(p) {
  editProject.value = p
  editForm.value    = { name: p.name, platform: p.platform }
}

async function saveEdit() {
  if (!editForm.value.name.trim()) return
  try {
    await projectStore.update(editProject.value.id, editForm.value)
    editProject.value = null
  } catch {
    toast.error('Failed to save changes')
  }
}

// ── Rotate API key ────────────────────────────────────────────────────────────
async function rotateKey(p) {
  if (!confirm(`Rotate the API key for "${p.name}"?\nAll SDKs using the old key will stop working until updated.`)) return
  try {
    await projectStore.rotateKey(p.id)
    toast.success('API key rotated successfully')
  } catch {
    toast.error('Failed to rotate API key')
  }
}

// ── Delete ────────────────────────────────────────────────────────────────────
function confirmDelete(p) { deleteTarget.value = p }

async function doDelete() {
  try {
    await projectStore.remove(deleteTarget.value.id)
    deleteTarget.value = null
  } catch {
    toast.error('Failed to delete project')
    deleteTarget.value = null
  }
}

// ── Alerts ────────────────────────────────────────────────────────────────────
async function openAlerts(p) {
  alertsProject.value = p
  alertForm.value     = { channel: 'webhook', endpoint: '' }
  try {
    const { data } = await axios.get(`/api/projects/${p.id}/alerts`)
    alerts.value = data.data
  } catch {
    toast.error('Failed to load alerts')
  }
}

async function addAlert() {
  if (!alertForm.value.endpoint.trim()) return
  try {
    const { data } = await axios.post(`/api/projects/${alertsProject.value.id}/alerts`, alertForm.value)
    alerts.value.unshift(data)
    alertForm.value.endpoint = ''
  } catch {
    toast.error('Failed to add alert')
  }
}

async function toggleAlert(a) {
  try {
    const { data } = await axios.patch(`/api/alerts/${a.id}`, { enabled: !a.enabled })
    const idx = alerts.value.findIndex(x => x.id === a.id)
    if (idx !== -1) alerts.value[idx] = { ...alerts.value[idx], ...data }
  } catch {
    toast.error('Failed to update alert')
  }
}

async function deleteAlertItem(a) {
  try {
    await axios.delete(`/api/alerts/${a.id}`)
    alerts.value = alerts.value.filter(x => x.id !== a.id)
  } catch {
    toast.error('Failed to delete alert')
  }
}

// ── Helpers ───────────────────────────────────────────────────────────────────
function dsn(key)  { return `${window.location.origin}/api/ingest/${key}` }
function copy(key) { navigator.clipboard.writeText(dsn(key)); toast.success('DSN copied') }

const alertPlaceholder = computed(() => ({
  webhook:  'https://your-server.com/hook',
  telegram: 'BOT_TOKEN:CHAT_ID',
  email:    'alerts@yourcompany.com',
})[alertForm.value.channel] ?? '')

const channelLabel = (ch) => ({ webhook: 'Webhook (HTTP POST)', telegram: 'Telegram', email: 'Email' })[ch] ?? ch

const channelIcon = (ch) => ({
  webhook:  { icon: '🔗', color: 'text-blue-400' },
  telegram: { icon: '✈️', color: 'text-sky-400' },
  email:    { icon: '📧', color: 'text-amber-400' },
})[ch] ?? { icon: '📡', color: 'text-gray-400' }
</script>

<style scoped>
.modal-enter-active, .modal-leave-active { transition: opacity 0.15s ease; }
.modal-enter-from, .modal-leave-to { opacity: 0; }
</style>
