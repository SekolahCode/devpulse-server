<template>
  <div class="max-w-6xl mx-auto px-6 py-8">

    <!-- Header -->
    <div class="flex items-center justify-between mb-5">
      <div class="flex items-center gap-3">
        <router-link to="/" class="text-gray-500 hover:text-gray-300 text-sm transition-colors">
          Projects
        </router-link>
        <span class="text-gray-700">/</span>
        <h1 class="text-[15px] font-semibold text-white">Issues</h1>
      </div>

      <!-- Status tabs -->
      <div class="flex items-center bg-[#111119] border border-white/6 rounded-lg p-1 gap-0.5">
        <button
          v-for="s in ['unresolved', 'resolved', 'ignored']"
          :key="s"
          @click="setStatus(s)"
          :class="status === s
            ? 'bg-violet-600 text-white shadow'
            : 'text-gray-400 hover:text-gray-200'"
          class="px-3 py-1 rounded-md text-xs font-medium capitalize transition-all"
        >
          {{ s }}
        </button>
      </div>
    </div>

    <!-- Stats bar -->
    <div v-if="store.stats" class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-5">
      <div class="bg-[#111119] border border-white/6 rounded-xl px-4 py-3 flex flex-col gap-0.5">
        <span class="text-[10px] text-gray-500 uppercase tracking-wide font-medium">Unresolved</span>
        <span class="text-xl font-bold text-red-400">{{ store.stats.issues.unresolved }}</span>
      </div>
      <div class="bg-[#111119] border border-white/6 rounded-xl px-4 py-3 flex flex-col gap-0.5">
        <span class="text-[10px] text-gray-500 uppercase tracking-wide font-medium">New 24 h</span>
        <span class="text-xl font-bold text-amber-400">{{ store.stats.issues.new_24h }}</span>
      </div>
      <div class="bg-[#111119] border border-white/6 rounded-xl px-4 py-3 flex flex-col gap-0.5">
        <span class="text-[10px] text-gray-500 uppercase tracking-wide font-medium">Regressions</span>
        <span class="text-xl font-bold text-orange-400">{{ store.stats.issues.regressions_24h }}</span>
      </div>
      <div class="bg-[#111119] border border-white/6 rounded-xl px-4 py-3 flex flex-col gap-0.5">
        <span class="text-[10px] text-gray-500 uppercase tracking-wide font-medium">Events 24 h</span>
        <span class="text-xl font-bold text-violet-400">{{ store.stats.events_24h }}</span>
      </div>
    </div>

    <!-- Search -->
    <div class="mb-4">
      <input
        v-model="search"
        @input="onSearch"
        type="text"
        placeholder="Search issues…"
        class="w-full bg-[#111119] border border-white/6 rounded-xl px-4 py-2.5 text-sm text-gray-200
               placeholder-gray-600 focus:outline-none focus:border-violet-500/50 transition-colors"
      />
    </div>

    <!-- Loading skeleton -->
    <div v-if="store.loading" class="space-y-2">
      <div v-for="i in 5" :key="i" class="h-14 bg-[#111119] rounded-xl animate-pulse" />
    </div>

    <!-- Empty state -->
    <div v-else-if="!store.issues.length" class="flex flex-col items-center justify-center py-24 text-center">
      <div class="text-3xl mb-3">{{ search ? '🔍' : '🎉' }}</div>
      <p class="text-gray-300 font-medium">
        {{ search ? 'No issues matching "' + search + '"' : 'No ' + status + ' issues' }}
      </p>
      <p class="text-gray-500 text-sm mt-1">{{ search ? 'Try a different search term' : 'This project is clean' }}</p>
    </div>

    <!-- Issue table -->
    <div v-else class="rounded-xl border border-white/6 overflow-hidden">
      <!-- Table header -->
      <div class="grid grid-cols-[1fr_auto_auto_auto] gap-4 px-5 py-2.5 bg-[#0d0d16] border-b border-white/5
                  text-[11px] font-medium text-gray-500 uppercase tracking-wide">
        <span>Issue</span>
        <span class="text-right">Events</span>
        <span class="text-right w-20">Last seen</span>
        <span class="w-20"></span>
      </div>

      <!-- Rows -->
      <div class="divide-y divide-white/4">
        <router-link
          v-for="issue in store.issues"
          :key="issue.id"
          :to="`/issues/${issue.id}`"
          class="grid grid-cols-[1fr_auto_auto_auto] gap-4 items-center px-5 py-3.5
                 bg-[#111119] hover:bg-[#14141e] transition-colors group"
        >
          <!-- Title + level -->
          <div class="flex items-center gap-3 min-w-0">
            <span :class="levelBadge(issue.level)"
                  class="text-[10px] font-bold px-1.5 py-0.5 rounded-md uppercase tracking-wide shrink-0">
              {{ issue.level }}
            </span>
            <p class="text-sm text-gray-200 truncate group-hover:text-white transition-colors">
              {{ issue.title }}
            </p>
          </div>

          <!-- Event count -->
          <span class="text-xs text-gray-500 tabular-nums text-right shrink-0">
            {{ issue.event_count.toLocaleString() }}×
          </span>

          <!-- Last seen -->
          <span class="text-xs text-gray-500 text-right shrink-0 w-20">
            {{ timeAgo(issue.last_seen) }}
          </span>

          <!-- Actions -->
          <div class="flex items-center gap-2 shrink-0 w-20" @click.prevent>
            <button
              @click="store.resolve(issue.id)"
              class="text-[11px] text-gray-500 hover:text-emerald-400 font-medium transition-colors px-1"
            >
              Resolve
            </button>
            <button
              @click="store.ignore(issue.id)"
              class="text-[11px] text-gray-500 hover:text-gray-300 transition-colors px-1"
            >
              Ignore
            </button>
          </div>
        </router-link>
      </div>

      <!-- Load more / footer -->
      <div class="px-5 py-3 bg-[#0d0d16] border-t border-white/5 flex items-center justify-between">
        <span class="text-xs text-gray-600">
          Showing {{ store.issues.length }} of {{ store.total }}
        </span>
        <button
          v-if="store.hasMore"
          @click="loadMore"
          :disabled="store.loadingMore"
          class="text-xs text-violet-400 hover:text-violet-300 disabled:opacity-50 transition-colors font-medium"
        >
          {{ store.loadingMore ? 'Loading…' : 'Load more' }}
        </button>
      </div>
    </div>

  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { useIssuesStore } from '../stores/issues'

const route  = useRoute()
const store  = useIssuesStore()
const status = ref('unresolved')
const search = ref('')

let searchTimer = null

onMounted(() => {
  store.fetchStats()
  store.fetch(route.params.id, status.value)
})

function setStatus(s) {
  status.value = s
  store.fetch(route.params.id, s, { search: search.value })
}

function onSearch() {
  clearTimeout(searchTimer)
  searchTimer = setTimeout(() => {
    store.fetch(route.params.id, status.value, { search: search.value })
  }, 300)
}

function loadMore() {
  store.fetchMore(route.params.id, status.value, search.value)
}

const levelBadge = (level) =>
  ({ error: 'bg-red-500/15 text-red-400', warning: 'bg-amber-500/15 text-amber-400', info: 'bg-blue-500/15 text-blue-400' })[level]
  ?? 'bg-gray-700/50 text-gray-400'

function timeAgo(date) {
  const diff = Math.floor((Date.now() - new Date(date)) / 1000)
  if (diff < 60)    return `${diff}s ago`
  if (diff < 3600)  return `${Math.floor(diff / 60)}m ago`
  if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`
  return `${Math.floor(diff / 86400)}d ago`
}
</script>
