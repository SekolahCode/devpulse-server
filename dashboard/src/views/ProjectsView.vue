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

    <!-- Empty state -->
    <div v-if="!projects.length" class="flex flex-col items-center justify-center py-24 text-center">
      <div class="w-12 h-12 rounded-full bg-violet-500/10 flex items-center justify-center text-2xl mb-4">📦</div>
      <p class="text-gray-300 font-medium">No projects yet</p>
      <p class="text-gray-500 text-sm mt-1">Create your first project to start tracking errors</p>
    </div>

    <!-- Grid -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <router-link
        v-for="p in projects"
        :key="p.id"
        :to="`/projects/${p.id}/issues`"
        class="group block bg-[#111119] border border-white/6 rounded-xl p-5 hover:border-violet-500/50 hover:bg-[#14141e] transition-all"
      >
        <div class="flex items-start justify-between gap-3">
          <div class="flex items-center gap-3 min-w-0">
            <div :class="platformColor(p.platform)" class="w-9 h-9 rounded-lg flex items-center justify-center text-lg shrink-0">
              {{ platformIcon(p.platform) }}
            </div>
            <div class="min-w-0">
              <h2 class="font-semibold text-white text-[15px] truncate group-hover:text-violet-300 transition-colors">{{ p.name }}</h2>
              <p class="text-xs text-gray-500 mt-0.5 capitalize">{{ p.platform }}</p>
            </div>
          </div>
          <span class="text-gray-600 group-hover:text-violet-400 transition-colors text-lg shrink-0">→</span>
        </div>

        <!-- DSN -->
        <div class="mt-4 flex items-center gap-2 bg-black/20 rounded-lg px-3 py-2">
          <span class="text-[10px] text-gray-500 uppercase tracking-wide font-medium shrink-0">DSN</span>
          <code class="text-[11px] text-gray-400 truncate font-mono">{{ dsn(p.api_key) }}</code>
          <button
            @click.prevent="copy(p.api_key)"
            class="ml-auto text-gray-600 hover:text-gray-300 text-xs shrink-0 transition-colors"
            title="Copy DSN"
          >⎘</button>
        </div>
      </router-link>
    </div>

    <!-- Create Project Modal -->
    <Transition name="modal">
      <div
        v-if="showCreate"
        class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 px-4"
        @click.self="showCreate = false"
      >
        <div class="bg-[#13131c] border border-white/8 rounded-2xl p-6 w-full max-w-sm shadow-2xl">
          <h2 class="text-lg font-semibold text-white mb-5">New Project</h2>
          <div class="space-y-3">
            <div>
              <label class="block text-xs text-gray-400 mb-1.5 font-medium">Project name</label>
              <input
                v-model="form.name"
                placeholder="e.g. My Laravel App"
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2.5 text-sm text-white placeholder-gray-600 focus:outline-none focus:border-violet-500 transition-colors"
              />
            </div>
            <div>
              <label class="block text-xs text-gray-400 mb-1.5 font-medium">Platform</label>
              <select
                v-model="form.platform"
                class="w-full bg-[#0b0b12] border border-white/8 rounded-lg px-3 py-2.5 text-sm text-white focus:outline-none focus:border-violet-500 transition-colors"
              >
                <option value="laravel">Laravel</option>
                <option value="wordpress">WordPress</option>
                <option value="php">PHP</option>
              </select>
            </div>
          </div>
          <div class="flex gap-3 mt-6">
            <button
              @click="create"
              :disabled="!form.name.trim()"
              class="flex-1 bg-violet-600 hover:bg-violet-500 disabled:opacity-40 disabled:cursor-not-allowed text-white py-2.5 rounded-lg text-sm font-medium transition-colors"
            >
              Create project
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

  </div>
</template>

<script setup>
import axios from 'axios'
import { onMounted, ref } from 'vue'

const projects  = ref([])
const showCreate = ref(false)
const form = ref({ name: '', platform: 'laravel' })

onMounted(async () => {
  const { data } = await axios.get('/api/projects')
  projects.value = data.data
})

async function create() {
  if (!form.value.name.trim()) return
  const { data } = await axios.post('/api/projects', form.value)
  projects.value.unshift(data)
  showCreate.value = false
  form.value = { name: '', platform: 'laravel' }
}

function dsn(key) {
  return `${window.location.origin}/api/ingest/${key}`
}

function copy(key) {
  navigator.clipboard.writeText(dsn(key))
}

const platformIcon  = (p) => ({ laravel: '🔴', wordpress: '🔵', php: '🟣' })[p] ?? '⬜'
const platformColor = (p) => ({ laravel: 'bg-red-500/10', wordpress: 'bg-blue-500/10', php: 'bg-purple-500/10' })[p] ?? 'bg-gray-500/10'
</script>

<style scoped>
.modal-enter-active, .modal-leave-active { transition: opacity 0.2s ease; }
.modal-enter-from, .modal-leave-to { opacity: 0; }
</style>
