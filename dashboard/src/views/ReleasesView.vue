<template>
  <div class="releases-root flex-1 w-full">
    <div class="max-w-4xl mx-auto px-6 py-8">

      <!-- Header -->
      <div class="flex flex-wrap items-center justify-between gap-3 mb-8">
        <div class="flex items-center gap-3">
          <router-link to="/" class="text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] text-sm transition-colors">Projects</router-link>
          <span class="text-[var(--dp-rule)]">/</span>
          <router-link :to="`/projects/${route.params.id}/issues`" class="text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] text-sm transition-colors">Issues</router-link>
          <span class="text-[var(--dp-rule)]">/</span>
          <h1 class="dp-display text-[15px] font-semibold text-[var(--dp-ink)]">Releases</h1>
        </div>
        <button
          @click="showCreate = true"
          class="flex items-center gap-2 whitespace-nowrap bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] active:translate-y-px text-white px-4 py-2 rounded-md text-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--dp-accent)]"
        >
          <span class="text-base leading-none">+</span>
          New Release
        </button>
      </div>

      <!-- Loading -->
      <div v-if="loading" class="space-y-3">
        <div v-for="i in 4" :key="i" class="h-20 bg-[var(--dp-surface-2)] rounded-lg animate-pulse" />
      </div>

      <!-- Empty -->
      <div v-else-if="!releases.length" class="flex flex-col items-center justify-center py-24 text-center">
        <div class="w-12 h-12 rounded-full bg-[var(--dp-accent-soft)] flex items-center justify-center text-2xl mb-4">🏷️</div>
        <p class="text-[var(--dp-ink)] font-medium">No releases yet</p>
        <p class="text-[var(--dp-ink-2)] text-sm mt-1">Track deployments and correlate them with issues</p>
      </div>

      <!-- Timeline -->
      <div v-else class="relative">
        <!-- Timeline line -->
        <div class="absolute left-[19px] top-5 bottom-5 w-px bg-[var(--dp-rule)] pointer-events-none" />

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
                  ? 'bg-[var(--dp-accent)] border-[var(--dp-accent)]'
                  : 'bg-[var(--dp-surface)] border-[var(--dp-rule)] group-hover:border-[var(--dp-ink-3)]'
              ]" />
            </div>

            <!-- Card -->
            <div class="dv-card flex-1 bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-lg p-4 hover:bg-[var(--dp-surface-2)]/60 transition-colors mb-1">
              <div class="flex items-start justify-between gap-3 mb-3">
                <!-- Version tag -->
                <div class="flex items-center gap-2.5">
                  <span :class="versionBadge(rel.version)"
                        class="dp-mono text-[11px] font-bold px-2 py-0.5 rounded-md tracking-wide">
                    v{{ rel.version }}
                  </span>
                  <span v-if="idx === 0" class="text-[9px] font-bold px-1.5 py-0.5 rounded bg-[var(--dp-accent-soft)] text-[var(--dp-accent)] uppercase tracking-wider">
                    Latest
                  </span>
                </div>
                <!-- Actions -->
                <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 focus-within:opacity-100 transition-opacity">
                  <a
                    v-if="rel.url"
                    :href="rel.url"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="w-6 h-6 flex items-center justify-center rounded text-[var(--dp-ink-3)] hover:text-blue-600 hover:bg-blue-500/10 transition-colors"
                    title="Open deploy URL"
                  >
                    <svg width="11" height="11" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M7 3H3v10h10V9M9 1h6v6M15 1l-8 8"/>
                    </svg>
                  </a>
                  <button
                    @click="openUpload(rel)"
                    class="w-6 h-6 flex items-center justify-center rounded text-[var(--dp-ink-3)] hover:text-[var(--dp-accent)] hover:bg-[var(--dp-accent-soft)] transition-colors"
                    title="Upload source map"
                  >
                    <svg width="11" height="11" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M8 11V3M8 3L5 6M8 3l3 3"/><path d="M2 11v2a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-2"/>
                    </svg>
                  </button>
                  <button
                    @click="confirmDelete(rel)"
                    class="w-6 h-6 flex items-center justify-center rounded text-[var(--dp-ink-3)] hover:text-red-600 hover:bg-red-500/10 transition-colors"
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
                <span v-if="rel.ref" class="flex items-center gap-1 dp-mono text-[11px] text-[var(--dp-ink-2)]">
                  <svg width="11" height="11" viewBox="0 0 16 16" fill="currentColor" class="text-[var(--dp-ink-3)]">
                    <path d="M11.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zm-2.25.75a2.25 2.25 0 1 1 3 2.122V6A2.5 2.5 0 0 1 10 8.5H6a1 1 0 0 0-1 1v1.128a2.251 2.251 0 1 1-1.5 0V5.372a2.25 2.25 0 1 1 1.5 0v1.836A2.492 2.492 0 0 1 6 7h4a1 1 0 0 0 1-1v-.628A2.25 2.25 0 0 1 9.5 3.25zM4.25 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zM4.25 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5z"/>
                  </svg>
                  {{ rel.ref.slice(0, 8) }}
                </span>
                <span class="dp-mono text-[11px] text-[var(--dp-ink-3)] tabular-nums">
                  {{ formatDate(rel.deployed_at) }}
                </span>
              </div>

              <!-- Issue stats -->
              <div class="flex items-center gap-4 flex-wrap">
                <router-link
                  v-if="rel.new_issues > 0"
                  :to="`/projects/${route.params.id}/issues?release=${rel.version}`"
                  class="flex items-center gap-1.5 dp-mono text-[11px] text-red-600 hover:text-red-700 transition-colors tabular-nums"
                  title="New issues introduced in this release"
                >
                  <span class="w-1.5 h-1.5 rounded-full bg-red-500" />
                  {{ rel.new_issues }} new {{ rel.new_issues === 1 ? 'issue' : 'issues' }}
                </router-link>
                <router-link
                  v-if="rel.open_issues > 0"
                  :to="`/projects/${route.params.id}/issues?release=${rel.version}`"
                  class="flex items-center gap-1.5 dp-mono text-[11px] text-amber-600 hover:text-amber-700 transition-colors tabular-nums"
                  title="Issues still open in this release"
                >
                  <span class="w-1.5 h-1.5 rounded-full bg-amber-500" />
                  {{ rel.open_issues }} open
                </router-link>
                <span
                  v-if="rel.resolved_issues > 0"
                  class="flex items-center gap-1.5 dp-mono text-[11px] text-emerald-700 tabular-nums"
                  title="Issues resolved by this release"
                >
                  <span class="w-1.5 h-1.5 rounded-full bg-emerald-500" />
                  {{ rel.resolved_issues }} resolved
                </span>
                <span v-if="!rel.new_issues && !rel.open_issues && !rel.resolved_issues"
                      class="text-[11px] text-[var(--dp-ink-3)] italic">
                  No tracked issues
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- ─── Create Release Modal ────────────────────────────────────────── -->
      <Transition name="modal">
        <div
          v-if="showCreate"
          class="fixed inset-0 bg-[var(--dp-scrim)] backdrop-blur-sm flex items-center justify-center z-50 px-4"
          @click.self="showCreate = false"
        >
          <div class="dv-modal bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-lg p-6 w-full max-w-sm">
            <h2 class="dp-display text-lg font-semibold text-[var(--dp-ink)] mb-5">New Release</h2>
            <div class="space-y-3">
              <div>
                <label class="block text-xs text-[var(--dp-ink-2)] mb-1.5 font-medium">Version <span class="text-red-600">*</span></label>
                <input
                  v-model="form.version"
                  placeholder="e.g. 1.4.2 or 2.0.0-beta.1"
                  class="dp-mono w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5 text-sm text-[var(--dp-ink)]
                         placeholder-[var(--dp-ink-3)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors"
                />
              </div>
              <div>
                <label class="block text-xs text-[var(--dp-ink-2)] mb-1.5 font-medium">Git ref / commit SHA <span class="text-[var(--dp-ink-3)]">(optional)</span></label>
                <input
                  v-model="form.ref"
                  placeholder="abc1234 or refs/tags/v1.4.2"
                  class="dp-mono w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5 text-sm text-[var(--dp-ink)]
                         placeholder-[var(--dp-ink-3)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors"
                />
              </div>
              <div>
                <label class="block text-xs text-[var(--dp-ink-2)] mb-1.5 font-medium">Deploy URL <span class="text-[var(--dp-ink-3)]">(optional)</span></label>
                <input
                  v-model="form.url"
                  placeholder="https://ci.example.com/builds/123"
                  class="w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5 text-sm text-[var(--dp-ink)]
                         placeholder-[var(--dp-ink-3)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors"
                />
              </div>
            </div>
            <p v-if="createError" class="text-xs text-red-600 mt-2">{{ createError }}</p>
            <div class="flex gap-3 mt-5">
              <button
                @click="createRelease"
                :disabled="!form.version.trim() || creating"
                class="flex-1 bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] disabled:opacity-40 disabled:cursor-not-allowed
                       text-white py-2.5 rounded-md text-sm font-medium transition-colors"
              >
                {{ creating ? 'Creating…' : 'Create release' }}
              </button>
              <button
                @click="showCreate = false"
                class="flex-1 bg-[var(--dp-surface-2)] hover:bg-[var(--dp-rule)] text-[var(--dp-ink-2)] py-2.5 rounded-md text-sm transition-colors"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      </Transition>

      <!-- ─── Upload Source Map Modal ─────────────────────────────────────── -->
      <Transition name="modal">
        <div
          v-if="uploadTarget"
          class="fixed inset-0 bg-[var(--dp-scrim)] backdrop-blur-sm flex items-center justify-center z-50 px-4"
          @click.self="uploadTarget = null"
        >
          <div class="dv-modal bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-lg p-6 w-full max-w-sm">
            <h2 class="dp-display text-lg font-semibold text-[var(--dp-ink)] mb-1">Upload source map</h2>
            <p class="text-xs text-[var(--dp-ink-2)] mb-5">
              For <span class="dp-mono text-[var(--dp-ink)]">v{{ uploadTarget.version }}</span> — deobfuscates
              minified JS stack traces for events tagged with this release.
            </p>
            <div class="space-y-3">
              <div>
                <label class="block text-xs text-[var(--dp-ink-2)] mb-1.5 font-medium">
                  Artifact name <span class="text-red-600">*</span>
                </label>
                <input
                  v-model="uploadForm.artifactName"
                  placeholder="the exact `file` path/URL as it appears in stack frames"
                  class="dp-mono w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3 py-2.5 text-sm text-[var(--dp-ink)]
                         placeholder-[var(--dp-ink-3)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors"
                />
              </div>
              <div>
                <label class="block text-xs text-[var(--dp-ink-2)] mb-1.5 font-medium">
                  Source map file (.map) <span class="text-red-600">*</span>
                </label>
                <input
                  type="file"
                  accept=".map,application/json"
                  @change="handleFileChange"
                  class="w-full text-sm text-[var(--dp-ink-2)] file:mr-3 file:py-2 file:px-3 file:rounded-md file:border-0
                         file:bg-[var(--dp-surface-2)] file:text-[var(--dp-ink-2)] file:text-xs hover:file:bg-[var(--dp-rule)] file:transition-colors"
                />
              </div>
            </div>
            <p v-if="uploadError" class="text-xs text-red-600 mt-2">{{ uploadError }}</p>
            <p v-if="uploadSuccess" class="text-xs text-emerald-600 mt-2">Uploaded — new events will show resolved frames.</p>
            <div class="flex gap-3 mt-5">
              <button
                @click="submitUpload"
                :disabled="!uploadForm.artifactName.trim() || !uploadForm.file || uploading"
                class="flex-1 bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] disabled:opacity-40 disabled:cursor-not-allowed
                       text-white py-2.5 rounded-md text-sm font-medium transition-colors"
              >
                {{ uploading ? 'Uploading…' : 'Upload' }}
              </button>
              <button
                @click="uploadTarget = null"
                class="flex-1 bg-[var(--dp-surface-2)] hover:bg-[var(--dp-rule)] text-[var(--dp-ink-2)] py-2.5 rounded-md text-sm transition-colors"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      </Transition>

      <!-- ─── Delete Confirm Modal ────────────────────────────────────────── -->
      <Transition name="modal">
        <div
          v-if="deleteTarget"
          class="fixed inset-0 bg-[var(--dp-scrim)] backdrop-blur-sm flex items-center justify-center z-50 px-4"
          @click.self="deleteTarget = null"
        >
          <div class="dv-modal bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-lg p-6 w-full max-w-sm">
            <h2 class="dp-display text-lg font-semibold text-[var(--dp-ink)] mb-2">Delete release?</h2>
            <p class="text-sm text-[var(--dp-ink-2)] mb-6">
              Remove <span class="dp-mono text-[var(--dp-ink)]">v{{ deleteTarget.version }}</span> from the timeline.
              Issue linkage will be preserved but the release entry will be gone.
            </p>
            <div class="flex gap-3">
              <button
                @click="doDelete"
                class="flex-1 bg-red-600 hover:bg-red-500 text-white py-2.5 rounded-md text-sm font-medium transition-colors"
              >
                Delete
              </button>
              <button
                @click="deleteTarget = null"
                class="flex-1 bg-[var(--dp-surface-2)] hover:bg-[var(--dp-rule)] text-[var(--dp-ink-2)] py-2.5 rounded-md text-sm transition-colors"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      </Transition>

    </div>
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
const uploadTarget  = ref(null)
const uploading     = ref(false)
const uploadError   = ref('')
const uploadSuccess = ref(false)

const form = ref({ version: '', ref: '', url: '' })
const uploadForm = ref({ artifactName: '', file: null })

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

function openUpload(rel) {
  uploadTarget.value  = rel
  uploadForm.value    = { artifactName: '', file: null }
  uploadError.value   = ''
  uploadSuccess.value = false
}

function handleFileChange(e) {
  uploadForm.value.file = e.target.files?.[0] ?? null
}

async function submitUpload() {
  if (!uploadForm.value.artifactName.trim() || !uploadForm.value.file) return
  uploading.value     = true
  uploadError.value   = ''
  uploadSuccess.value = false
  try {
    const text = await uploadForm.value.file.text()
    const sourceMap = JSON.parse(text)
    await axios.post(
      `/api/projects/${route.params.id}/releases/${encodeURIComponent(uploadTarget.value.version)}/sourcemaps`,
      { artifact_name: uploadForm.value.artifactName.trim(), source_map: sourceMap }
    )
    uploadSuccess.value = true
  } catch (err) {
    uploadError.value = err instanceof SyntaxError
      ? 'That file is not valid JSON — expected a .map source map.'
      : (err.response?.data?.error ?? 'Failed to upload source map')
  } finally {
    uploading.value = false
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
  if (!v) return 'bg-gray-500/10 text-gray-600'
  const pre = v.includes('-') || v.includes('+')
  if (pre) return 'bg-gray-500/10 text-gray-600'
  const parts = v.split('.').map(Number)
  const [major, minor, patch] = parts
  if (isNaN(major)) return 'bg-gray-500/10 text-gray-600'
  if (minor === 0 && patch === 0) return 'bg-red-500/10 text-red-700'      // x.0.0 — major
  if (patch === 0)                return 'bg-blue-500/10 text-blue-700'    // x.y.0 — minor
  return 'bg-emerald-500/10 text-emerald-700'                             // x.y.z — patch
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
/* Hallmark · macrostructure: Dossier Timeline (was deployment timeline) · genre: modern-minimal (formal/exclusive)
 * theme: DevPulse locked system (design.md) — formal white workspace, oxblood accent
 * display: Fraunces · body: Geist · outlier(mono): JetBrains Mono
 * hairline spine, quieter record cards — see design.md § Macrostructure family
 */
.releases-root {
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

.dv-card {
  box-shadow: none;
}

.dv-modal {
  box-shadow: 0 12px 32px oklch(20% 0.02 40 / 16%);
}

.modal-enter-active, .modal-leave-active { transition: opacity 0.15s ease; }
.modal-enter-from, .modal-leave-to { opacity: 0; }

@media (prefers-reduced-motion: reduce) {
  .modal-enter-active, .modal-leave-active { transition-duration: 0.01s; }
}
</style>
