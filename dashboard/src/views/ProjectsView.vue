<template>
  <div class="projects-root flex-1 w-full">
    <div class="max-w-5xl mx-auto px-6 py-10">

      <!-- Page header -->
      <div class="flex flex-wrap items-center justify-between gap-3 mb-8">
        <div>
          <h1 class="dp-display text-2xl font-semibold text-[var(--dp-ink)] tracking-tight">Projects</h1>
          <p class="text-sm text-[var(--dp-ink-2)] mt-0.5">
            {{ projectStore.loaded ? countLine : 'Loading…' }}
          </p>
        </div>
        <button
          @click="showCreate = true"
          class="flex items-center gap-2 whitespace-nowrap shrink-0 bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] active:translate-y-px text-white px-5 py-2.5 rounded-md text-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--dp-accent)]"
        >
          <span class="text-base leading-none">+</span>
          New Project
        </button>
      </div>

      <!-- Loading skeleton -->
      <div v-if="!projectStore.loaded" class="border-t border-[var(--dp-rule)]">
        <div v-for="i in 4" :key="i" class="h-[4.5rem] border-b border-[var(--dp-rule)] px-1 flex items-center gap-4">
          <div class="w-9 h-9 rounded-md bg-[var(--dp-surface-2)] animate-pulse shrink-0" />
          <div class="h-3.5 w-40 rounded bg-[var(--dp-surface-2)] animate-pulse" />
        </div>
      </div>

      <!-- Empty state -->
      <div v-else-if="!projectStore.projects.length" class="flex flex-col items-center justify-center py-24 text-center border-t border-[var(--dp-rule)]">
        <div class="w-12 h-12 rounded-md bg-[var(--dp-accent-soft)] flex items-center justify-center text-2xl mb-4">📦</div>
        <p class="dp-display text-[var(--dp-ink)] text-lg">No projects yet</p>
        <p class="text-[var(--dp-ink-2)] text-sm mt-1">Create your first project to start tracking errors</p>
        <button
          @click="showCreate = true"
          class="mt-5 bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] text-white px-5 py-2.5 rounded-md text-sm font-medium transition-colors"
        >
          Create a project
        </button>
      </div>

      <!-- Ledger row list — hairline dividers, no boxed panel -->
      <div v-else class="border-t border-[var(--dp-rule)]">

        <!-- Column heads (desktop) -->
        <div class="hidden md:grid dp-row-grid px-1 py-2.5 border-b border-[var(--dp-rule)]">
          <span class="dp-mono text-[10px] uppercase tracking-widest text-[var(--dp-ink-3)]">Project</span>
          <span class="dp-mono text-[10px] uppercase tracking-widest text-[var(--dp-ink-3)] text-right">Unresolved</span>
          <span class="dp-mono text-[10px] uppercase tracking-widest text-[var(--dp-ink-3)] text-right">Last activity</span>
          <span class="dp-mono text-[10px] uppercase tracking-widest text-[var(--dp-ink-3)] text-center">DSN</span>
          <span class="dp-mono text-[10px] uppercase tracking-widest text-[var(--dp-ink-3)] text-right">Manage</span>
        </div>

        <!-- Rows -->
        <div
          v-for="p in projectStore.projects"
          :key="p.id"
          class="group grid dp-row-grid-mobile md:dp-row-grid items-center gap-y-2 px-1 py-5 border-b border-[var(--dp-rule)] hover:bg-[var(--dp-surface-2)]/40 transition-colors"
        >
          <!-- Name cell → issues -->
          <router-link :to="`/projects/${p.id}/issues`"
            class="flex items-center gap-3 min-w-0 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--dp-accent)] rounded">
            <div :class="platformColor(p.platform)" class="w-9 h-9 rounded-md flex items-center justify-center text-lg shrink-0">
              {{ platformIcon(p.platform) }}
            </div>
            <div class="min-w-0">
              <span class="dp-display block text-[var(--dp-ink)] text-[16px] truncate group-hover:text-[var(--dp-accent)] transition-colors">
                {{ p.name }}
              </span>
              <span class="block text-xs text-[var(--dp-ink-3)] capitalize">{{ p.platform }}</span>
            </div>
          </router-link>

          <!-- Unresolved count -->
          <div class="justify-self-start md:justify-self-end">
            <span v-if="health[p.id]?.state === 'loading'"
              class="inline-block h-4 w-8 rounded bg-[var(--dp-surface-2)] animate-pulse align-middle" />
            <router-link v-else-if="(health[p.id]?.total ?? 0) > 0"
              :to="`/projects/${p.id}/issues`"
              class="dp-mono inline-flex items-center gap-1 text-[12px] font-medium text-[var(--dp-danger)] bg-[var(--dp-danger-soft)] px-2 py-0.5 rounded-full tabular-nums hover:opacity-80 transition-opacity">
              ▲ {{ health[p.id].total }}
            </router-link>
            <span v-else-if="health[p.id]?.state === 'done'"
              class="dp-mono text-[12px] text-[var(--dp-ink-3)] tabular-nums">0 · quiet</span>
            <span v-else class="dp-mono text-[12px] text-[var(--dp-ink-3)]">—</span>
          </div>

          <!-- Last activity -->
          <span class="dp-mono text-[12px] text-[var(--dp-ink-2)] md:text-right tabular-nums">
            {{ health[p.id]?.state === 'done' ? timeAgo(health[p.id].lastSeen) : '…' }}
          </span>

          <!-- DSN copy -->
          <div class="md:justify-self-center">
            <button @click="copy(p.api_key)" :title="dsn(p.api_key)"
              class="dp-mono inline-flex items-center gap-1.5 text-[11px] text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] border border-[var(--dp-rule)] hover:border-[var(--dp-ink-3)] rounded-md px-2.5 py-1 transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]">
              ⎘ <span class="whitespace-nowrap">{{ copiedKey === p.api_key ? 'Copied ✓' : 'Copy DSN' }}</span>
            </button>
          </div>

          <!-- Manage actions -->
          <div class="flex items-center gap-1 md:justify-self-end">
            <button @click="openAlerts(p)" title="Manage alerts"
              class="w-7 h-7 flex items-center justify-center rounded text-[var(--dp-ink-3)] hover:text-amber-600 hover:bg-amber-500/10 transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]">
              <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                <path d="M8 1a5.5 5.5 0 0 0-5.5 5.5v2.293l-1.146 1.146A.5.5 0 0 0 1.5 11h13a.5.5 0 0 0 .354-.854L13.5 8.793V6.5A5.5 5.5 0 0 0 8 1zM6.5 13a1.5 1.5 0 0 0 3 0H6.5z"/>
              </svg>
            </button>
            <button @click="rotateKey(p)" title="Rotate API key"
              class="w-7 h-7 flex items-center justify-center rounded text-[var(--dp-ink-3)] hover:text-cyan-600 hover:bg-cyan-500/10 transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]">
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/>
              </svg>
            </button>
            <button @click="startEdit(p)" title="Edit project"
              class="w-7 h-7 flex items-center justify-center rounded text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-rule)] transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]">
              <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <path d="M11.5 2.5l2 2L5 13H3v-2L11.5 2.5z"/>
              </svg>
            </button>
            <button @click="confirmDelete(p)" title="Delete project"
              class="w-7 h-7 flex items-center justify-center rounded text-[var(--dp-ink-3)] hover:text-red-600 hover:bg-red-500/10 transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]">
              <svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <path d="M2 4h12M5 4V2h6v2M6 7v5M10 7v5M3 4l1 9h8l1-9"/>
              </svg>
            </button>
          </div>
        </div>
      </div>

      <!-- ─── Create Project Modal ────────────────────────────────────────── -->
      <Transition name="modal">
        <div v-if="showCreate"
          class="fixed inset-0 bg-[var(--dp-scrim)] backdrop-blur-sm flex items-center justify-center z-50 px-4"
          @click.self="showCreate = false">
          <div class="dp-modal bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-lg p-6 w-full max-w-sm">
            <h2 class="dp-display text-lg font-semibold text-[var(--dp-ink)] mb-5">New Project</h2>
            <div class="space-y-3">
              <div>
                <label class="block text-xs text-[var(--dp-ink-2)] mb-1.5 font-medium">Project name</label>
                <input v-model="createForm.name" placeholder="e.g. My Laravel App" autofocus
                  class="w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5 text-sm text-[var(--dp-ink)] placeholder-[var(--dp-ink-3)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors" />
              </div>
              <div>
                <label class="block text-xs text-[var(--dp-ink-2)] mb-1.5 font-medium">Platform</label>
                <select v-model="createForm.platform"
                  class="w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5 text-sm text-[var(--dp-ink)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors">
                  <option v-for="p in PLATFORMS" :key="p" :value="p" class="capitalize">{{ p }}</option>
                </select>
              </div>
            </div>
            <p v-if="createError" class="text-[12px] text-red-600 mt-3">{{ createError }}</p>
            <div class="flex gap-3 mt-6">
              <button @click="create" :disabled="!createForm.name.trim()"
                class="flex-1 bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] disabled:opacity-40 disabled:cursor-not-allowed text-white py-2.5 rounded-md text-sm font-medium transition-colors">
                Create project
              </button>
              <button @click="showCreate = false"
                class="flex-1 bg-[var(--dp-surface-2)] hover:bg-[var(--dp-rule)] text-[var(--dp-ink-2)] py-2.5 rounded-md text-sm transition-colors">
                Cancel
              </button>
            </div>
          </div>
        </div>
      </Transition>

      <!-- ─── Edit Project Modal ──────────────────────────────────────────── -->
      <Transition name="modal">
        <div v-if="editProject"
          class="fixed inset-0 bg-[var(--dp-scrim)] backdrop-blur-sm flex items-center justify-center z-50 px-4"
          @click.self="editProject = null">
          <div class="dp-modal bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-lg p-6 w-full max-w-sm">
            <h2 class="dp-display text-lg font-semibold text-[var(--dp-ink)] mb-5">Edit Project</h2>
            <div class="space-y-3">
              <div>
                <label class="block text-xs text-[var(--dp-ink-2)] mb-1.5 font-medium">Project name</label>
                <input v-model="editForm.name"
                  class="w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5 text-sm text-[var(--dp-ink)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors" />
              </div>
              <div>
                <label class="block text-xs text-[var(--dp-ink-2)] mb-1.5 font-medium">Platform</label>
                <select v-model="editForm.platform"
                  class="w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5 text-sm text-[var(--dp-ink)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors">
                  <option v-for="p in PLATFORMS" :key="p" :value="p" class="capitalize">{{ p }}</option>
                </select>
              </div>
            </div>
            <div class="flex gap-3 mt-6">
              <button @click="saveEdit" :disabled="!editForm.name.trim()"
                class="flex-1 bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] disabled:opacity-40 disabled:cursor-not-allowed text-white py-2.5 rounded-md text-sm font-medium transition-colors">
                Save changes
              </button>
              <button @click="editProject = null"
                class="flex-1 bg-[var(--dp-surface-2)] hover:bg-[var(--dp-rule)] text-[var(--dp-ink-2)] py-2.5 rounded-md text-sm transition-colors">
                Cancel
              </button>
            </div>
          </div>
        </div>
      </Transition>

      <!-- ─── Delete Confirm Modal ────────────────────────────────────────── -->
      <Transition name="modal">
        <div v-if="deleteTarget"
          class="fixed inset-0 bg-[var(--dp-scrim)] backdrop-blur-sm flex items-center justify-center z-50 px-4"
          @click.self="deleteTarget = null">
          <div class="dp-modal bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-lg p-6 w-full max-w-sm">
            <h2 class="dp-display text-lg font-semibold text-[var(--dp-ink)] mb-2">Delete project?</h2>
            <p class="text-sm text-[var(--dp-ink-2)] mb-6">
              This will permanently delete <span class="text-[var(--dp-ink)] font-medium">{{ deleteTarget.name }}</span>
              and all its issues and events. This cannot be undone.
            </p>
            <div class="flex gap-3">
              <button @click="doDelete"
                class="flex-1 bg-red-600 hover:bg-red-500 text-white py-2.5 rounded-md text-sm font-medium transition-colors">
                Delete permanently
              </button>
              <button @click="deleteTarget = null"
                class="flex-1 bg-[var(--dp-surface-2)] hover:bg-[var(--dp-rule)] text-[var(--dp-ink-2)] py-2.5 rounded-md text-sm transition-colors">
                Cancel
              </button>
            </div>
          </div>
        </div>
      </Transition>

      <!-- ─── Alerts Modal ────────────────────────────────────────────────── -->
      <Transition name="modal">
        <div v-if="alertsProject"
          class="fixed inset-0 bg-[var(--dp-scrim)] backdrop-blur-sm flex items-center justify-center z-50 px-4"
          @click.self="alertsProject = null">
          <div class="dp-modal bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-lg p-6 w-full max-w-lg">
            <div class="flex items-center justify-between mb-5">
              <h2 class="dp-display text-lg font-semibold text-[var(--dp-ink)]">
                Alerts — <span class="text-[var(--dp-accent)]">{{ alertsProject.name }}</span>
              </h2>
              <button @click="alertsProject = null" class="text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] text-lg leading-none">✕</button>
            </div>

            <div v-if="alerts.length" class="space-y-2 mb-4">
              <div v-for="a in alerts" :key="a.id"
                class="flex items-center gap-3 bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5">
                <span :class="channelIcon(a.channel).color" class="text-lg shrink-0">{{ channelIcon(a.channel).icon }}</span>
                <div class="min-w-0 flex-1">
                  <p class="dp-mono text-[12px] text-[var(--dp-ink-2)] truncate">{{ a.endpoint }}</p>
                  <p class="text-[10px] text-[var(--dp-ink-3)] capitalize">{{ a.channel }} · {{ a.cooldown_minutes ?? 60 }}m cooldown</p>
                </div>
                <button @click="toggleAlert(a)"
                  :class="a.enabled ? 'bg-emerald-500/15 text-emerald-700' : 'bg-[var(--dp-surface-2)] text-[var(--dp-ink-3)]'"
                  class="text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wide transition-colors">
                  {{ a.enabled ? 'ON' : 'OFF' }}
                </button>
                <button @click="deleteAlertItem(a)"
                  class="w-6 h-6 flex items-center justify-center rounded text-[var(--dp-ink-3)] hover:text-red-600 hover:bg-red-500/10 transition-colors shrink-0">
                  <svg width="11" height="11" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                    <line x1="3" y1="3" x2="13" y2="13"/><line x1="13" y1="3" x2="3" y2="13"/>
                  </svg>
                </button>
              </div>
            </div>
            <p v-else class="text-sm text-[var(--dp-ink-2)] mb-4">No alerts configured for this project.</p>

            <div class="border-t border-[var(--dp-rule)] pt-4">
              <p class="text-[11px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium mb-3">Add Alert</p>
              <div class="space-y-2">
                <select v-model="alertForm.channel"
                  class="w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2 text-sm text-[var(--dp-ink)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors">
                  <option v-for="ch in CHANNELS" :key="ch" :value="ch">{{ channelLabel(ch) }}</option>
                </select>
                <input v-model="alertForm.endpoint" :placeholder="alertPlaceholder"
                  class="dp-mono w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2 text-sm text-[var(--dp-ink)] placeholder-[var(--dp-ink-3)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors" />
                <button @click="addAlert" :disabled="!alertForm.endpoint.trim()"
                  class="w-full bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] disabled:opacity-40 disabled:cursor-not-allowed text-white py-2 rounded-md text-sm font-medium transition-colors">
                  Add alert
                </button>
              </div>
            </div>
          </div>
        </div>
      </Transition>

    </div>
  </div>
</template>

<script setup>
import axios from 'axios'
import { computed, onMounted, onUnmounted, reactive, ref } from 'vue'
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
const copiedKey     = ref(null)

const createForm = ref({ name: '', platform: 'laravel' })
const editForm   = ref({ name: '', platform: 'laravel' })
const alertForm  = ref({ channel: 'webhook', endpoint: '' })

// ── Per-project health (unresolved count + last activity) ───────────────────
// Uses the existing issues endpoint: total = unresolved count (status defaults
// to "unresolved"), first row's last_seen = most recent activity. One tiny
// request per project — no backend changes needed.
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

const countLine = computed(() => {
  const n = projectStore.projects.length
  const broken = projectStore.projects.filter(p => (health[p.id]?.total ?? 0) > 0).length
  if (!n) return 'No projects yet'
  const base = `${n} project${n === 1 ? '' : 's'}`
  return broken > 0 ? `${base} · ${broken} with unresolved issues` : base
})

function timeAgo(iso) {
  if (!iso) return '—'
  const s = (Date.now() - new Date(iso).getTime()) / 1000
  if (s < 60)        return 'just now'
  if (s < 3600)      return `${Math.floor(s / 60)}m ago`
  if (s < 86400)     return `${Math.floor(s / 3600)}h ago`
  if (s < 86400 * 30) return `${Math.floor(s / 86400)}d ago`
  return new Date(iso).toLocaleDateString('en', { month: 'short', day: 'numeric' })
}

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
  try {
    await projectStore.load()
    loadHealth(projectStore.projects)
  }
  catch { toast.error('Failed to load projects') }
})
onUnmounted(() => document.removeEventListener('keydown', onKeydown))

// ── Create ────────────────────────────────────────────────────────────────────
async function create() {
  if (!createForm.value.name.trim()) return
  createError.value = ''
  try {
    const created = await projectStore.create(createForm.value)
    showCreate.value = false
    createForm.value = { name: '', platform: 'laravel' }
    if (created?.id) loadHealth([created])
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

let copiedTimer = null
function copy(key) {
  navigator.clipboard.writeText(dsn(key))
  copiedKey.value = key
  clearTimeout(copiedTimer)
  copiedTimer = setTimeout(() => { copiedKey.value = null }, 2500)
}

const alertPlaceholder = computed(() => ({
  webhook:  'https://your-server.com/hook',
  telegram: 'BOT_TOKEN:CHAT_ID',
  email:    'alerts@yourcompany.com',
})[alertForm.value.channel] ?? '')

const channelLabel = (ch) => ({ webhook: 'Webhook (HTTP POST)', telegram: 'Telegram', email: 'Email' })[ch] ?? ch

const channelIcon = (ch) => ({
  webhook:  { icon: '🔗', color: 'text-blue-600' },
  telegram: { icon: '✈️', color: 'text-sky-600' },
  email:    { icon: '📧', color: 'text-amber-600' },
})[ch] ?? { icon: '📡', color: 'text-gray-500' }
</script>

<style scoped>
/* Hallmark · macrostructure: Ledger Row · genre: modern-minimal (formal/exclusive)
 * design-system: design.md · designed-as-app · nav: N3 side-rail (App.vue)
 * formal white workspace, oxblood accent · counts via /api/issues?limit=1
 * rows sit on hairline dividers, no boxed panel — see design.md § Macrostructure family
 * pre-emit critique: P4 H5 E4 S4 R5 V4
 */
.projects-root {
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

/* Row grid: name | unresolved | last activity | dsn | actions */
@media (min-width: 48rem) {
  .dp-row-grid,
  .md\:dp-row-grid {
    display: grid;
    grid-template-columns: minmax(0, 1fr) 6.5rem 7.5rem 8rem auto;
    column-gap: 1rem;
    align-items: center;
  }
}

/* Mobile: name full row, stats wrap beneath */
@media (max-width: 47.99rem) {
  .dp-row-grid-mobile {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    column-gap: 0.75rem;
  }
  .dp-row-grid-mobile > :first-child { grid-column: 1 / -1; }
}

.dp-modal {
  box-shadow: 0 12px 32px oklch(20% 0.02 40 / 16%);
}

.modal-enter-active, .modal-leave-active { transition: opacity 0.15s ease; }
.modal-enter-from, .modal-leave-to { opacity: 0; }

@media (prefers-reduced-motion: reduce) {
  .modal-enter-active, .modal-leave-active { transition-duration: 0.01s; }
}
</style>
