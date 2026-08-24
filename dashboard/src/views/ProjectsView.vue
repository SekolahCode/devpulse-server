<template>
  <div class="projects-root flex-1 w-full">
    <div class="px-6 py-8">

      <!-- Page header -->
      <div class="flex flex-wrap items-center justify-between gap-3 mb-6">
        <div>
          <h1 class="text-2xl font-semibold text-[var(--dp-ink)] tracking-tight">Projects</h1>
          <p class="text-sm text-[var(--dp-ink-2)] mt-0.5">
            {{ projectStore.loaded ? countLine : 'Loading…' }}
          </p>
        </div>
        <button
          @click="showCreate = true"
          class="flex items-center gap-2 whitespace-nowrap shrink-0 bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] active:translate-y-px text-white px-4 py-2.5 rounded-xl text-sm font-medium shadow-sm transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--dp-accent)]"
        >
          <Plus :size="16" />
          New Project
        </button>
      </div>

      <!-- Loading skeleton -->
      <div v-if="!projectStore.loaded" class="overflow-hidden rounded-2xl border border-[var(--dp-rule)] bg-[var(--dp-surface)]">
        <div v-for="i in 4" :key="i" class="h-[4.5rem] border-b border-[var(--dp-rule)] last:border-0 px-5 flex items-center gap-4">
          <div class="w-9 h-9 rounded-full bg-[var(--dp-surface-2)] animate-pulse shrink-0" />
          <div class="h-3.5 w-40 rounded bg-[var(--dp-surface-2)] animate-pulse" />
        </div>
      </div>

      <!-- Empty state -->
      <div v-else-if="!projectStore.projects.length" class="flex flex-col items-center justify-center py-24 text-center rounded-2xl border border-[var(--dp-rule)] bg-[var(--dp-surface)]">
        <div class="w-12 h-12 rounded-full bg-[var(--dp-accent-soft)] flex items-center justify-center text-2xl mb-4">📦</div>
        <p class="text-[var(--dp-ink)] text-lg font-medium">No projects yet</p>
        <p class="text-[var(--dp-ink-2)] text-sm mt-1">Create your first project to start tracking errors</p>
        <button
          @click="showCreate = true"
          class="mt-5 flex items-center gap-2 bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] text-white px-4 py-2.5 rounded-xl text-sm font-medium shadow-sm transition-colors"
        >
          <Plus :size="16" />
          Create a project
        </button>
      </div>

      <!-- Boxed table — matches the reference DataTable: rounded-2xl border, washed header, divide-y rows -->
      <div v-else class="overflow-hidden rounded-2xl border border-[var(--dp-rule)] bg-[var(--dp-surface)]">

        <!-- Column heads (desktop) — sortable via header buttons -->
        <div class="hidden md:grid dp-row-grid px-5 py-3 bg-[var(--dp-surface-2)]/60 border-b border-[var(--dp-rule)]">
          <button type="button" @click="handleSort('name')"
            class="inline-flex items-center gap-1 text-xs font-medium text-[var(--dp-ink-3)] hover:text-[var(--dp-accent)] transition-colors">
            Project
            <ArrowUp v-if="sortKey === 'name' && sortDirection === 'asc'" :size="12" />
            <ArrowDown v-else-if="sortKey === 'name' && sortDirection === 'desc'" :size="12" />
            <ArrowUpDown v-else :size="12" class="text-[var(--dp-rule)]" />
          </button>
          <button type="button" @click="handleSort('unresolved')"
            class="inline-flex items-center justify-end gap-1 text-xs font-medium text-[var(--dp-ink-3)] hover:text-[var(--dp-accent)] transition-colors">
            Unresolved
            <ArrowUp v-if="sortKey === 'unresolved' && sortDirection === 'asc'" :size="12" />
            <ArrowDown v-else-if="sortKey === 'unresolved' && sortDirection === 'desc'" :size="12" />
            <ArrowUpDown v-else :size="12" class="text-[var(--dp-rule)]" />
          </button>
          <button type="button" @click="handleSort('last_activity')"
            class="inline-flex items-center justify-end gap-1 text-xs font-medium text-[var(--dp-ink-3)] hover:text-[var(--dp-accent)] transition-colors">
            Last activity
            <ArrowUp v-if="sortKey === 'last_activity' && sortDirection === 'asc'" :size="12" />
            <ArrowDown v-else-if="sortKey === 'last_activity' && sortDirection === 'desc'" :size="12" />
            <ArrowUpDown v-else :size="12" class="text-[var(--dp-rule)]" />
          </button>
          <span class="text-xs font-medium text-[var(--dp-ink-3)] text-center">DSN</span>
          <span class="text-xs font-medium text-[var(--dp-ink-3)] text-right">Manage</span>
        </div>

        <!-- Rows -->
        <div
          v-for="p in sortedProjects"
          :key="p.id"
          class="group grid dp-row-grid-mobile md:dp-row-grid items-center gap-y-2 px-5 py-4 border-b border-[var(--dp-rule)] last:border-0 last:rounded-b-2xl hover:bg-[var(--dp-surface-2)]/40 transition-colors"
        >
          <!-- Name cell → issues -->
          <router-link :to="`/projects/${p.id}/issues`"
            class="flex items-center gap-3 min-w-0 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--dp-accent)] rounded">
            <div :class="platformColor(p.platform)" class="w-9 h-9 rounded-full flex items-center justify-center shrink-0">
              <img :src="platformIcon(p.platform)" :alt="p.platform" class="w-5 h-5" />
            </div>
            <div class="min-w-0">
              <span class="block text-[var(--dp-ink)] text-sm font-medium truncate group-hover:text-[var(--dp-accent)] transition-colors">
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
              class="dp-mono inline-flex items-center gap-1.5 text-[11px] text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)] border border-[var(--dp-rule)] hover:border-[var(--dp-ink-3)] rounded-lg px-2.5 py-1 transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]">
              <Copy :size="12" />
              <span class="whitespace-nowrap">{{ copiedKey === p.api_key ? 'Copied ✓' : 'Copy DSN' }}</span>
            </button>
          </div>

          <!-- Manage actions — DropdownMenuContent teleports to <body>, so it
               floats above the boxed table instead of being clipped by its
               overflow-hidden (which a plain absolute-positioned div was). -->
          <div class="flex items-center justify-center md:justify-self-end">
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <button title="Manage project"
                  class="w-8 h-8 flex items-center justify-center rounded-lg text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)] transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)] data-[state=open]:text-[var(--dp-ink)] data-[state=open]:bg-[var(--dp-surface-2)]">
                  <MoreVertical :size="16" />
                </button>
              </DropdownMenuTrigger>
              <DropdownMenuContent class="w-52">
                <DropdownMenuItem @click="openAlerts(p)">
                  <Bell :size="13" class="shrink-0" />
                  Manage alerts
                </DropdownMenuItem>
                <DropdownMenuItem @click="rotateKey(p)">
                  <RotateCcw :size="13" class="shrink-0" />
                  Rotate API key
                </DropdownMenuItem>
                <DropdownMenuItem @click="startEdit(p)">
                  <Pencil :size="13" class="shrink-0" />
                  Edit project
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem variant="destructive" @click="confirmDelete(p)">
                  <Trash2 :size="13" class="shrink-0" />
                  Delete project
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
      </div>

      <!-- ─── Create Project Modal ────────────────────────────────────────── -->
      <Modal :open="showCreate" title="New project" @close="showCreate = false">
        <div class="space-y-3">
          <div>
            <label class="dp-label">Project name</label>
            <input v-model="createForm.name" placeholder="e.g. My Laravel App" autofocus class="dp-input" />
          </div>
          <div>
            <label class="dp-label">Platform</label>
            <Select v-model="createForm.platform">
              <SelectTrigger class="dp-select-trigger capitalize">
                <SelectValue />
              </SelectTrigger>
              <SelectContent class="bg-[var(--dp-surface)] border-[var(--dp-rule)] text-[var(--dp-ink)]">
                <SelectItem v-for="p in PLATFORMS" :key="p" :value="p" class="dp-select-item capitalize">{{ p }}</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <p v-if="createError" class="text-[12px] text-red-600">{{ createError }}</p>
        </div>
        <template #footer>
          <button @click="create" :disabled="!createForm.name.trim()" class="dp-btn-primary">
            Create project
          </button>
          <button @click="showCreate = false" class="dp-btn-secondary">Cancel</button>
        </template>
      </Modal>

      <!-- ─── Edit Project Modal ──────────────────────────────────────────── -->
      <Modal :open="!!editProject" title="Edit project" @close="editProject = null">
        <div class="space-y-3">
          <div>
            <label class="dp-label">Project name</label>
            <input v-model="editForm.name" class="dp-input" />
          </div>
          <div>
            <label class="dp-label">Platform</label>
            <Select v-model="editForm.platform">
              <SelectTrigger class="dp-select-trigger capitalize">
                <SelectValue />
              </SelectTrigger>
              <SelectContent class="bg-[var(--dp-surface)] border-[var(--dp-rule)] text-[var(--dp-ink)]">
                <SelectItem v-for="p in PLATFORMS" :key="p" :value="p" class="dp-select-item capitalize">{{ p }}</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
        <template #footer>
          <button @click="saveEdit" :disabled="!editForm.name.trim()" class="dp-btn-primary">
            Save changes
          </button>
          <button @click="editProject = null" class="dp-btn-secondary">Cancel</button>
        </template>
      </Modal>

      <!-- ─── Rotate API Key Modal ────────────────────────────────────────── -->
      <Modal :open="!!rotateTarget" title="Rotate API key?" @close="rotateTarget = null">
        <template #description>
          Every SDK still using <span class="text-[var(--dp-ink)] font-medium">{{ rotateTarget?.name }}</span>'s
          current key will stop reporting until you update it with the new one.
        </template>
        <template #footer>
          <button @click="doRotate" :disabled="rotating" class="dp-btn-primary">
            {{ rotating ? 'Rotating…' : 'Rotate key' }}
          </button>
          <button @click="rotateTarget = null" class="dp-btn-secondary">Cancel</button>
        </template>
      </Modal>

      <!-- ─── Delete Confirm Modal ────────────────────────────────────────── -->
      <Modal :open="!!deleteTarget" title="Delete project?" @close="deleteTarget = null">
        <template #description>
          This permanently deletes <span class="text-[var(--dp-ink)] font-medium">{{ deleteTarget?.name }}</span>
          and every issue and event under it. This cannot be undone.
        </template>
        <div>
          <label class="dp-label">
            Type <span class="dp-mono text-[var(--dp-ink)]">{{ deleteTarget?.name }}</span> to confirm
          </label>
          <input v-model="deleteConfirmName" :placeholder="deleteTarget?.name" autofocus
            class="dp-input" @keyup.enter="deleteConfirmed && doDelete()" />
        </div>
        <template #footer>
          <button @click="doDelete" :disabled="!deleteConfirmed"
            class="flex-1 bg-red-600 hover:bg-red-500 disabled:opacity-40 disabled:cursor-not-allowed text-white py-2.5 rounded-xl text-sm font-medium transition-colors">
            Delete permanently
          </button>
          <button @click="deleteTarget = null" class="dp-btn-secondary">Cancel</button>
        </template>
      </Modal>

      <!-- ─── Alerts Modal ────────────────────────────────────────────────── -->
      <Modal :open="!!alertsProject" size="lg" @close="alertsProject = null">
        <template #title>
          Alerts — <span class="text-[var(--dp-accent)]">{{ alertsProject?.name }}</span>
        </template>

        <div v-if="alerts.length" class="space-y-2 mb-4">
          <div v-for="a in alerts" :key="a.id"
            class="flex items-center gap-3 bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-xl px-3 py-2.5">
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
              class="w-6 h-6 flex items-center justify-center rounded-lg text-[var(--dp-ink-3)] hover:text-red-600 hover:bg-red-500/10 transition-colors shrink-0">
              <Trash2 :size="12" />
            </button>
          </div>
        </div>
        <p v-else class="text-sm text-[var(--dp-ink-2)] mb-4">No alerts configured for this project.</p>

        <div class="border-t border-[var(--dp-rule)] pt-4">
          <p class="text-[11px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium mb-3">Add alert</p>
          <div class="space-y-2">
            <Select v-model="alertForm.channel">
              <SelectTrigger class="dp-select-trigger">
                <SelectValue />
              </SelectTrigger>
              <SelectContent class="bg-[var(--dp-surface)] border-[var(--dp-rule)] text-[var(--dp-ink)]">
                <SelectItem v-for="ch in CHANNELS" :key="ch" :value="ch" class="dp-select-item">
                  {{ channelLabel(ch) }}
                </SelectItem>
              </SelectContent>
            </Select>
            <input v-model="alertForm.endpoint" :placeholder="alertPlaceholder" class="dp-input dp-mono" />
            <button @click="addAlert" :disabled="!alertForm.endpoint.trim()"
              class="w-full bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] disabled:opacity-40 disabled:cursor-not-allowed text-white py-2.5 rounded-xl text-sm font-medium transition-colors">
              Add alert
            </button>
          </div>
        </div>
      </Modal>

    </div>
  </div>
</template>

<script setup>
import axios from 'axios'
import { computed, onMounted, onUnmounted, reactive, ref } from 'vue'
import { ArrowUp, ArrowDown, ArrowUpDown, Plus, Bell, RotateCcw, Pencil, Trash2, Copy, MoreVertical } from 'lucide-vue-next'
import { Modal } from '@/components/ui/modal'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'
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
const rotateTarget  = ref(null)
const rotating      = ref(false)

// Delete requires typing the exact project name — the action is irreversible
// and wipes every issue/event under the project, so a single click is too cheap.
const deleteConfirmName = ref('')
const deleteConfirmed = computed(() =>
  !!deleteTarget.value && deleteConfirmName.value.trim() === deleteTarget.value.name
)

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

// ── Sortable columns ──────────────────────────────────────────────────────────
const sortKey       = ref(null)
const sortDirection = ref('asc')

function handleSort(key) {
  sortDirection.value = sortKey.value === key && sortDirection.value === 'asc' ? 'desc' : 'asc'
  sortKey.value = key
}

const sortedProjects = computed(() => {
  if (!sortKey.value) return projectStore.projects
  const dir = sortDirection.value === 'asc' ? 1 : -1

  return [...projectStore.projects].sort((a, b) => {
    if (sortKey.value === 'name') {
      return a.name.localeCompare(b.name) * dir
    }
    if (sortKey.value === 'unresolved') {
      return ((health[a.id]?.total ?? -1) - (health[b.id]?.total ?? -1)) * dir
    }
    // last_activity
    const aTime = health[a.id]?.lastSeen ? new Date(health[a.id].lastSeen).getTime() : -1
    const bTime = health[b.id]?.lastSeen ? new Date(health[b.id].lastSeen).getTime() : -1
    return (aTime - bTime) * dir
  })
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
// (the Manage dropdown closes itself — reka-ui's DropdownMenu owns its own
// open state and outside-click/Escape handling.)
function onKeydown(e) {
  if (e.key !== 'Escape') return
  if (alertsProject.value) { alertsProject.value = null; return }
  if (deleteTarget.value)  { deleteTarget.value  = null; return }
  if (rotateTarget.value)  { rotateTarget.value  = null; return }
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
onUnmounted(() => {
  document.removeEventListener('keydown', onKeydown)
})

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
function rotateKey(p) { rotateTarget.value = p }

async function doRotate() {
  rotating.value = true
  try {
    await projectStore.rotateKey(rotateTarget.value.id)
    toast.success('API key rotated successfully')
    rotateTarget.value = null
  } catch {
    toast.error('Failed to rotate API key')
  } finally {
    rotating.value = false
  }
}

// ── Delete ────────────────────────────────────────────────────────────────────
function confirmDelete(p) {
  deleteTarget.value = p
  deleteConfirmName.value = ''
}

async function doDelete() {
  if (!deleteConfirmed.value) return
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
/* Hallmark · macrostructure: boxed table · genre: modern-minimal (clean/formal)
 * design-system: design.md · designed-as-app · nav: floating rail (App.vue)
 * slate/maroon workspace (accent moved off the teleradiology-matched indigo per a later request) · counts via /api/issues?limit=1
 * rows live in a rounded-2xl bordered box with divide-y hairlines — see design.md § Macrostructure family
 * pre-emit critique: P4 H5 E4 S4 R5 V4
 */
.projects-root {
  background: var(--dp-paper);
  font-family: var(--dp-font-body);
  color: var(--dp-ink);
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
    grid-template-columns: minmax(0, 1fr) 6.5rem 7.5rem 8rem 9rem;
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

/* Shared modal form/action styling — every dialog on this page renders through
   the same <Modal> shell, so these keep field and button treatment identical
   across create / edit / rotate / delete / alerts instead of per-modal classes. */
.dp-label {
  display: block;
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--dp-ink-2);
  margin-bottom: 0.375rem;
}

.dp-input {
  width: 100%;
  background: var(--dp-paper);
  border: 1px solid var(--dp-rule);
  border-radius: 0.75rem;
  padding: 0.625rem 0.75rem;
  font-size: 0.875rem;
  color: var(--dp-ink);
  transition: border-color 120ms, box-shadow 120ms;
}
.dp-input::placeholder { color: var(--dp-ink-3); }
.dp-input:focus {
  outline: none;
  border-color: var(--dp-accent);
  box-shadow: 0 0 0 2px color-mix(in oklch, var(--dp-accent) 15%, transparent);
}

/* shadcn Select styled to match .dp-input so a native-looking field and a
   custom dropdown never sit side by side in the same form. */
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

.dp-btn-primary,
.dp-btn-secondary {
  flex: 1;
  padding-block: 0.625rem;
  border-radius: 0.75rem;
  font-size: 0.875rem;
  transition: background-color 120ms, opacity 120ms;
}
.dp-btn-primary {
  background: var(--dp-accent);
  color: #fff;
  font-weight: 500;
}
.dp-btn-primary:hover:not(:disabled) { background: var(--dp-accent-hover); }
.dp-btn-primary:disabled { opacity: 0.4; cursor: not-allowed; }
.dp-btn-secondary {
  background: var(--dp-surface-2);
  color: var(--dp-ink-2);
}
.dp-btn-secondary:hover { background: var(--dp-rule); }
</style>
