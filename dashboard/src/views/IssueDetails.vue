<template>
  <div class="issue-detail-root flex-1 w-full">
    <div class="px-6 py-8">

      <!-- Breadcrumb -->
      <div class="flex items-center gap-2 mb-8">
        <button
          @click="router.back()"
          type="button"
          title="Back"
          class="w-8 h-8 -ml-1.5 flex items-center justify-center rounded-lg text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)] transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)] shrink-0"
        >
          <ArrowLeft :size="16" />
        </button>
        <Breadcrumb>
          <BreadcrumbList>
            <BreadcrumbItem>
              <BreadcrumbLink as-child>
                <router-link to="/projects">Projects</router-link>
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
              <BreadcrumbLink as-child>
                <button @click="router.back()">Issues</button>
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
              <BreadcrumbPage>Detail</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
      </div>

      <!-- Loading skeleton -->
      <div v-if="loading" class="space-y-6">
        <div class="h-8 w-2/3 bg-[var(--dp-surface-2)] rounded-lg animate-pulse" />
        <div class="grid grid-cols-2 md:grid-cols-5 gap-3">
          <div v-for="i in 5" :key="i" class="h-20 bg-[var(--dp-surface-2)] rounded-2xl animate-pulse" />
        </div>
        <div class="grid lg:grid-cols-3 gap-6">
          <div class="lg:col-span-2 h-64 bg-[var(--dp-surface-2)] rounded-2xl animate-pulse" />
          <div class="h-64 bg-[var(--dp-surface-2)] rounded-2xl animate-pulse" />
        </div>
      </div>

      <template v-else-if="issue">

        <!-- Title + metadata + actions -->
        <div class="flex flex-col lg:flex-row lg:items-start lg:justify-between gap-4 mb-6">
          <div class="flex items-start gap-3 min-w-0">
            <span :class="levelBadge(issue.level)"
                  class="text-[10px] font-bold px-1.5 py-0.5 rounded-full uppercase tracking-wide shrink-0 mt-1">
              {{ issue.level }}
            </span>
            <div class="min-w-0">
              <h1 class="text-xl font-semibold text-[var(--dp-ink)] leading-snug wrap-break-word">{{ issue.title }}</h1>
              <div class="flex flex-wrap items-center gap-2 mt-2.5">
                <span :class="priorityBadge(issue.priority)"
                      class="text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wide">
                  {{ issue.priority ?? 'medium' }} priority
                </span>
                <span v-if="issue.last_release" class="dp-mono text-[11px] text-[var(--dp-ink-2)] bg-[var(--dp-surface-2)] px-2 py-0.5 rounded">
                  v{{ issue.last_release }}
                </span>
                <div class="flex items-center gap-1.5">
                  <span v-if="issue.assignee" class="flex items-center gap-1 text-[11px] text-sky-700 bg-sky-500/10 px-2 py-0.5 rounded">
                    <User :size="10" />
                    {{ issue.assignee }}
                  </span>
                  <button v-if="!showAssignee" @click="showAssignee = true"
                    class="text-[11px] text-[var(--dp-ink-3)] hover:text-[var(--dp-ink-2)] transition-colors">
                    {{ issue.assignee ? 'Reassign' : '+ Assign' }}
                  </button>
                  <form v-else @submit.prevent="setAssignee" class="flex items-center gap-1">
                    <input v-model="assigneeInput" placeholder="username" autofocus
                      class="text-[11px] bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded px-2 py-0.5 text-[var(--dp-ink)] w-28 focus:outline-none focus:border-[var(--dp-accent)]" />
                    <button type="submit" class="text-[var(--dp-accent)] hover:opacity-80 px-1"><Check :size="12" /></button>
                    <button type="button" @click="showAssignee = false" class="text-[var(--dp-ink-3)] hover:text-[var(--dp-ink-2)] px-1"><X :size="12" /></button>
                  </form>
                </div>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex gap-2 shrink-0">
            <button @click="resolve"
              class="flex items-center gap-1.5 bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] active:translate-y-px text-white
                     px-4 py-2 rounded-xl text-sm font-medium shadow-sm transition-colors">
              <Check :size="14" />
              Resolve
            </button>
            <button @click="ignore"
              class="flex items-center gap-1.5 bg-[var(--dp-surface-2)] hover:bg-[var(--dp-rule)] text-[var(--dp-ink-2)]
                     px-4 py-2 rounded-xl text-sm transition-colors">
              <X :size="14" />
              Ignore
            </button>
          </div>
        </div>

        <!-- Stats grid -->
        <div class="grid grid-cols-2 md:grid-cols-5 gap-3 mb-6">
          <div class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
            <p class="text-[11px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium">Status</p>
            <p :class="statusColor(issue.status)" class="text-base font-semibold mt-1 capitalize">{{ issue.status }}</p>
          </div>
          <div class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
            <p class="text-[11px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium">Events</p>
            <p class="dp-mono tabular-nums text-base font-semibold text-[var(--dp-ink)] mt-1">{{ issue.event_count?.toLocaleString() ?? '—' }}</p>
          </div>
          <div class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
            <p class="text-[11px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium">Affected users</p>
            <p class="dp-mono tabular-nums text-base font-semibold text-amber-600 mt-1">{{ issue.affected_users?.toLocaleString() ?? '0' }}</p>
          </div>
          <div class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
            <p class="text-[11px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium">Last seen</p>
            <p class="dp-mono text-base font-semibold text-[var(--dp-ink)] mt-1">{{ timeAgo(issue.last_seen) }}</p>
          </div>
          <div class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
            <p class="text-[11px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium">First seen</p>
            <p class="dp-mono text-base font-semibold text-[var(--dp-ink)] mt-1">{{ timeAgo(issue.first_seen) }}</p>
          </div>
        </div>

        <!-- Main: occurrence readout (left) + occurrences / fingerprint (right) -->
        <div class="grid lg:grid-cols-3 gap-6 items-start">

          <!-- ── Left: selected occurrence ─────────────────────────────────── -->
          <div class="lg:col-span-2 min-w-0">

            <!-- Occurrence summary card -->
            <div v-if="selectedEvent" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl overflow-hidden mb-5">
              <div class="flex items-center justify-between gap-3 px-5 py-2.5 bg-[var(--dp-surface-2)]/60 border-b border-[var(--dp-rule)]">
                <div class="flex items-center gap-2">
                  <span class="text-[11px] text-[var(--dp-ink-3)] font-medium">{{ occurrenceLabel(selectedEventIdx) }}</span>
                  <span v-if="selectedEvent.environment"
                        class="text-[10px] px-1.5 py-0.5 rounded font-bold uppercase tracking-wide"
                        :class="envBadge(selectedEvent.environment)">
                    {{ selectedEvent.environment }}
                  </span>
                </div>
                <div class="flex items-center gap-2">
                  <span v-if="selectedEvent.release"
                        class="dp-mono text-[10px] text-[var(--dp-ink-3)] bg-[var(--dp-surface)] px-1.5 py-0.5 rounded">
                    v{{ selectedEvent.release }}
                  </span>
                  <span class="dp-mono text-[11px] text-[var(--dp-ink-3)] tabular-nums">{{ formatDate(selectedEvent.created_at) }}</span>
                </div>
              </div>

              <!-- Exception headline -->
              <div v-if="selectedEvent.payload?.exception" class="px-6 pt-5 pb-5">
                <h3 class="dp-mono text-2xl font-bold text-[var(--dp-ink)] mb-1">
                  {{ selectedEvent.payload.exception.type?.split('\\').pop() ?? 'Error' }}
                </h3>
                <p v-if="selectedEvent.payload.exception.stacktrace?.[0]?.file"
                   class="dp-mono text-[12px] text-[var(--dp-ink-3)] mb-3">
                  {{ selectedEvent.payload.exception.stacktrace[0].file }}:{{ selectedEvent.payload.exception.stacktrace[0].line }}
                </p>
                <p class="text-[14px] text-[var(--dp-ink-2)] leading-relaxed mb-4">
                  {{ selectedEvent.payload.exception.message }}
                </p>
                <div class="flex flex-wrap items-center gap-2">
                  <span v-if="selectedEvent.context?.laravel"
                        class="dp-mono text-[10px] font-bold px-2 py-0.5 rounded bg-purple-500/10 text-purple-700 tracking-wide">
                    LARAVEL {{ selectedEvent.context.laravel }}
                  </span>
                  <span v-if="selectedEvent.context?.php"
                        class="dp-mono text-[10px] font-bold px-2 py-0.5 rounded bg-blue-500/10 text-blue-700 tracking-wide">
                    PHP {{ selectedEvent.context.php }}
                  </span>
                  <span class="text-[10px] font-bold px-2 py-0.5 rounded bg-amber-500/10 text-amber-700 tracking-wide">
                    UNHANDLED
                  </span>
                  <span class="dp-mono text-[10px] font-bold px-2 py-0.5 rounded bg-[var(--dp-surface-2)] text-[var(--dp-ink-3)] tracking-wide">
                    CODE {{ selectedEvent.payload.exception.code ?? 0 }}
                  </span>
                  <span v-if="selectedEvent.payload?.request?.method"
                        :class="methodColor(selectedEvent.payload.request.method)"
                        class="dp-mono text-[10px] font-bold px-2 py-0.5 rounded tracking-wide">
                    {{ selectedEvent.payload.request.method }}
                  </span>
                  <span v-if="selectedEvent.payload?.request?.url"
                        class="dp-mono text-[11px] text-[var(--dp-ink-3)] truncate max-w-xs">
                    {{ selectedEvent.payload.request.url }}
                  </span>
                  <span v-if="selectedEvent.payload?.plugin"
                        :class="pluginBadgeColor(selectedEvent.payload.plugin.type)"
                        class="dp-mono text-[10px] font-bold px-2 py-0.5 rounded tracking-wide">
                    {{ pluginBadgeLabel(selectedEvent.payload.plugin) }}
                  </span>
                </div>
              </div>
              <div v-else class="px-6 py-5 text-[13px] text-[var(--dp-ink-2)]">
                {{ selectedEvent.payload?.message ?? 'No exception payload for this occurrence.' }}
              </div>
            </div>
            <div v-else class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl px-6 py-5 text-[13px] text-[var(--dp-ink-2)] mb-5">
              No occurrences recorded for this issue yet.
            </div>

            <!-- Tab bar -->
            <div class="flex items-center gap-1 bg-[var(--dp-surface-2)] border border-[var(--dp-rule)] rounded-xl p-1 mb-5 overflow-x-auto">
              <button v-for="t in TABS" :key="t.value" type="button" @click="activeTab = t.value"
                :class="activeTab === t.value ? 'bg-[var(--dp-accent)] text-white' : 'text-[var(--dp-ink-2)] hover:text-[var(--dp-ink)]'"
                class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium whitespace-nowrap transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]">
                <component :is="t.icon" :size="13" />
                {{ t.label }}
              </button>
            </div>

            <!-- ── Details tab ──────────────────────────────────────────────── -->
            <div v-if="activeTab === 'details' && selectedEvent" class="space-y-5">

              <!-- Performance Vitals -->
              <div v-if="isVitalsEvent(selectedEvent)" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl px-6 py-5">
                <p class="text-[11px] font-semibold text-[var(--dp-ink-2)] mb-5 flex items-center gap-2">
                  <Gauge :size="13" class="text-[var(--dp-accent)] shrink-0" />
                  Performance Vitals
                </p>
                <div class="grid grid-cols-3 sm:grid-cols-5 gap-4">
                  <div v-for="m in getVitals(selectedEvent)" :key="m.key" class="flex flex-col items-center gap-2.5">
                    <div class="relative w-15 h-15">
                      <svg viewBox="0 0 36 36" class="w-full h-full -rotate-90">
                        <circle cx="18" cy="18" r="15.9" fill="none" stroke="currentColor"
                                stroke-width="2.2" class="text-[var(--dp-rule)]"/>
                        <circle cx="18" cy="18" r="15.9" fill="none"
                                stroke-width="2.5" stroke-linecap="round"
                                :stroke="m.color"
                                :stroke-dasharray="`${m.score} 100`"/>
                      </svg>
                      <span class="dp-mono absolute inset-0 flex items-center justify-center text-[15px] font-bold"
                            :style="{ color: m.color }">
                        {{ m.score }}
                      </span>
                    </div>
                    <div class="text-center">
                      <p class="text-[11px] font-semibold text-[var(--dp-ink-2)] leading-tight">{{ m.label }}</p>
                      <p class="dp-mono text-[10px] text-[var(--dp-ink-3)] tabular-nums mt-0.5">{{ m.raw }} / 100</p>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Fatal error notice -->
              <div v-if="selectedEvent.payload?.is_fatal" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
                <div class="flex items-start gap-2 bg-amber-500/10 border border-amber-500/20 rounded-xl px-3 py-2.5">
                  <AlertTriangle :size="14" class="text-amber-700 shrink-0 mt-px" />
                  <div>
                    <p class="text-[11px] text-amber-800 font-semibold">
                      Fatal error{{ selectedEvent.payload.error_type ? ` (${selectedEvent.payload.error_type})` : '' }} — captured by shutdown handler
                    </p>
                    <p class="text-[11px] text-amber-700/80 mt-0.5">
                      PHP fatal errors terminate the script before the call stack is available.
                      Only the error origin file and line are shown above.
                    </p>
                  </div>
                </div>
              </div>

              <!-- Artisan command -->
              <div v-if="selectedEvent.payload?.command" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
                <p class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium mb-2">Artisan Command</p>
                <div class="flex flex-wrap items-center gap-2">
                  <span class="dp-mono text-[12px] text-emerald-700 bg-emerald-500/10 px-2 py-1 rounded">
                    php artisan {{ selectedEvent.payload.command }}
                  </span>
                  <span v-if="selectedEvent.payload.exit_code !== undefined"
                        class="dp-mono text-[11px] text-red-700 bg-red-500/10 px-2 py-0.5 rounded">
                    exit {{ selectedEvent.payload.exit_code }}
                  </span>
                  <span v-if="selectedEvent.payload.input?.trim()" class="dp-mono text-[11px] text-[var(--dp-ink-2)]">
                    {{ selectedEvent.payload.input }}
                  </span>
                </div>
                <div v-if="selectedEvent.payload.laravel || selectedEvent.payload.php" class="flex gap-2 mt-2">
                  <span v-if="selectedEvent.payload.laravel"
                        class="dp-mono text-[10px] text-purple-700 bg-purple-500/10 px-1.5 py-0.5 rounded">
                    Laravel {{ selectedEvent.payload.laravel }}
                  </span>
                  <span v-if="selectedEvent.payload.php"
                        class="dp-mono text-[10px] text-blue-700 bg-blue-500/10 px-1.5 py-0.5 rounded">
                    PHP {{ selectedEvent.payload.php }}
                  </span>
                </div>
              </div>

              <!-- User -->
              <div v-if="selectedEvent.payload?.user" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
                <p class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium mb-2">User</p>
                <div class="flex flex-wrap gap-2">
                  <span v-if="selectedEvent.payload.user.id"       class="dp-mono text-[11px] text-[var(--dp-ink-2)] bg-[var(--dp-surface-2)] px-2 py-0.5 rounded">ID: {{ selectedEvent.payload.user.id }}</span>
                  <span v-if="selectedEvent.payload.user.email"    class="text-[11px] text-[var(--dp-ink-2)] bg-[var(--dp-surface-2)] px-2 py-0.5 rounded">{{ selectedEvent.payload.user.email }}</span>
                  <span v-if="selectedEvent.payload.user.username" class="text-[11px] text-[var(--dp-ink-2)] bg-[var(--dp-surface-2)] px-2 py-0.5 rounded">{{ selectedEvent.payload.user.username }}</span>
                </div>
              </div>

              <!-- WordPress platform -->
              <div v-if="selectedEvent.context?.platform === 'wordpress'" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
                <p class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium mb-2">Platform</p>
                <div class="flex flex-wrap items-center gap-2 mb-2">
                  <span v-if="selectedEvent.context.wordpress" class="text-[11px] font-semibold px-2 py-0.5 rounded bg-blue-500/10 text-blue-700">
                    WP {{ selectedEvent.context.wordpress }}
                  </span>
                  <span v-if="selectedEvent.context.php" class="text-[11px] font-semibold px-2 py-0.5 rounded bg-purple-500/10 text-purple-700">
                    PHP {{ selectedEvent.context.php }}
                  </span>
                  <span v-if="selectedEvent.context.theme" class="text-[11px] text-[var(--dp-ink-2)] bg-[var(--dp-surface-2)] px-2 py-0.5 rounded">
                    {{ selectedEvent.context.theme }}
                  </span>
                  <span v-if="selectedEvent.context.memory" class="dp-mono text-[11px] text-[var(--dp-ink-3)] bg-[var(--dp-surface-2)] px-2 py-0.5 rounded">
                    {{ formatMemory(selectedEvent.context.memory) }} peak mem
                  </span>
                </div>
                <div class="flex flex-wrap gap-1.5 mb-3">
                  <span v-if="selectedEvent.context.is_admin"     class="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/10 text-amber-700 font-semibold">admin</span>
                  <span v-if="selectedEvent.context.multisite"    class="text-[10px] px-1.5 py-0.5 rounded bg-blue-500/10 text-blue-700 font-semibold">multisite</span>
                  <span v-if="selectedEvent.context.wp_debug"     class="text-[10px] px-1.5 py-0.5 rounded bg-orange-500/10 text-orange-700 font-semibold">WP_DEBUG</span>
                  <span v-if="selectedEvent.context.wp_debug_log" class="text-[10px] px-1.5 py-0.5 rounded bg-orange-500/10 text-orange-700 font-semibold">DEBUG_LOG</span>
                </div>

                <!-- Active plugins -->
                <div v-if="selectedEvent.context?.active_plugins?.length" class="pt-3 border-t border-[var(--dp-rule)]">
                  <div class="flex items-center justify-between mb-1.5">
                    <p class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium">Active plugins</p>
                    <button @click="togglePlugins(selectedEvent.id)"
                      class="flex items-center gap-1 text-[10px] text-[var(--dp-ink-3)] hover:text-[var(--dp-ink-2)] transition-colors tabular-nums">
                      {{ expandedPlugins.has(selectedEvent.id) ? 'hide' : `${selectedEvent.context.plugin_count ?? selectedEvent.context.active_plugins.length} plugins` }}
                      <ChevronDown :size="11" class="transition-transform" :class="expandedPlugins.has(selectedEvent.id) ? 'rotate-180' : ''" />
                    </button>
                  </div>
                  <div v-if="expandedPlugins.has(selectedEvent.id)"
                       class="dp-mono text-[11px] text-[var(--dp-ink-3)] space-y-0.5 max-h-36 overflow-y-auto">
                    <div v-for="p in selectedEvent.context.active_plugins" :key="p"
                         class="truncate hover:text-[var(--dp-ink-2)] transition-colors">{{ p }}</div>
                  </div>
                  <div v-else class="text-[11px] text-[var(--dp-ink-3)] italic">Click to expand</div>
                </div>
              </div>

              <p v-if="!hasDetailsContent" class="text-[13px] text-[var(--dp-ink-3)] py-10 text-center">
                No additional details captured for this occurrence.
              </p>
            </div>

            <!-- ── Stack Trace tab ──────────────────────────────────────────── -->
            <div v-else-if="activeTab === 'trace' && selectedEvent">
              <div v-if="selectedEvent.payload?.exception?.stacktrace?.length" class="space-y-1.5">
                <template v-for="(group, gi) in groupFrames(selectedEvent.payload.exception.stacktrace)">

                  <!-- App frame card -->
                  <div v-if="group.type === 'app'"
                       :key="`app-${gi}`"
                       class="border border-[var(--dp-rule)] rounded-xl overflow-hidden">
                    <div class="flex items-center justify-between gap-3 px-4 py-2.5 bg-[var(--dp-surface-2)]">
                      <div class="flex items-center gap-2.5 min-w-0 flex-1">
                        <span class="shrink-0 w-2 h-2 rounded-full"
                              :class="gi === 0 ? 'bg-red-500' : 'bg-[var(--dp-ink-3)]'"></span>
                        <span class="dp-mono text-[12px] truncate"
                              :class="gi === 0 ? 'text-[var(--dp-ink)]' : 'text-[var(--dp-ink-2)]'">
                          <span v-if="group.frame.function" class="text-[var(--dp-accent)]">{{ group.frame.function }}</span>
                          <span v-else-if="group.frame.file" class="text-[var(--dp-ink-3)]">{{ group.frame.file.split('/').pop() }}</span>
                          <span v-else class="text-[var(--dp-ink-3)] italic">unknown</span>
                        </span>
                      </div>
                      <div class="flex items-center gap-2 shrink-0">
                        <span v-if="group.frame.plugin && group.frame.plugin.type !== 'core'"
                              :class="pluginBadgeColor(group.frame.plugin.type)"
                              class="dp-mono text-[10px] px-1.5 py-0.5 rounded">
                          {{ group.frame.plugin.name }}
                        </span>
                        <span v-if="group.frame.resolved"
                              class="dp-mono text-[10px] px-1.5 py-0.5 rounded bg-emerald-500/10 text-emerald-700"
                              title="Resolved via uploaded source map">
                          mapped
                        </span>
                        <span class="dp-mono text-[11px] text-[var(--dp-ink-3)]">
                          {{ shortPath(group.frame.file) }}:{{ group.frame.line }}
                        </span>
                        <button v-if="group.frame.context"
                                @click="toggleFrame(selectedEventIdx, gi)"
                                class="w-5 h-5 flex items-center justify-center rounded border border-[var(--dp-rule)]
                                       hover:border-[var(--dp-ink-3)] text-[var(--dp-ink-3)] hover:text-[var(--dp-ink-2)] transition-colors shrink-0">
                          <component :is="collapsedFrames.has(`${selectedEventIdx}-${gi}`) ? Plus : Minus" :size="11" />
                        </button>
                      </div>
                    </div>
                    <div v-if="group.frame.context && !collapsedFrames.has(`${selectedEventIdx}-${gi}`)">
                      <div v-for="(codeLine, lineNum) in group.frame.context.lines"
                           :key="lineNum"
                           class="dp-mono flex items-stretch text-[12px] leading-6"
                           :class="Number(lineNum) === group.frame.line ? 'bg-red-500/10' : 'hover:bg-[var(--dp-surface-2)]/60'">
                        <span class="shrink-0 w-12 text-right pr-3 select-none tabular-nums py-0.5 border-r"
                              :class="Number(lineNum) === group.frame.line
                                ? 'text-red-700 font-bold border-red-500/40 bg-red-500/15'
                                : 'text-[var(--dp-ink-3)] border-[var(--dp-rule)]'">
                          {{ lineNum }}
                        </span>
                        <span class="pl-4 py-0.5 whitespace-pre overflow-x-auto flex-1"
                              :class="Number(lineNum) === group.frame.line ? 'text-red-800' : 'text-[var(--dp-ink-2)]'">{{ codeLine }}</span>
                      </div>
                    </div>
                  </div>

                  <!-- Vendor frames group -->
                  <div v-else :key="`vendor-${gi}`" class="border border-[var(--dp-rule)] rounded-xl overflow-hidden">
                    <button @click="toggleVendorGroup(selectedEventIdx, gi)"
                            class="w-full flex items-center gap-2.5 px-4 py-2 text-[11px] text-[var(--dp-ink-3)]
                                   hover:text-[var(--dp-ink-2)] hover:bg-[var(--dp-surface-2)]/60 transition-colors">
                      <Package :size="12" class="shrink-0" />
                      <span>{{ group.frames.length }} vendor frame{{ group.frames.length !== 1 ? 's' : '' }}</span>
                      <ChevronDown :size="12" class="ml-auto transition-transform duration-150"
                            :class="expandedVendorGroups.has(`${selectedEventIdx}-${gi}`) ? 'rotate-180' : ''" />
                    </button>
                    <div v-if="expandedVendorGroups.has(`${selectedEventIdx}-${gi}`)" class="border-t border-[var(--dp-rule)]">
                      <div v-for="(vf, vfi) in group.frames" :key="vfi"
                           class="flex items-start gap-3 px-4 py-1.5 border-b border-[var(--dp-rule)] last:border-0 hover:bg-[var(--dp-surface-2)]/40">
                        <span class="shrink-0 w-1.5 h-1.5 rounded-full bg-[var(--dp-ink-3)] mt-1.5"></span>
                        <span class="dp-mono text-[11px] text-[var(--dp-ink-3)] flex-1 truncate">
                          {{ vf.function ?? (vf.file?.split('/').pop() ?? 'unknown') }}
                        </span>
                        <span class="dp-mono text-[10px] text-[var(--dp-ink-3)] shrink-0">
                          {{ shortPath(vf.file) }}{{ vf.line ? `:${vf.line}` : '' }}
                        </span>
                      </div>
                    </div>
                  </div>

                </template>
              </div>
              <p v-else class="text-[13px] text-[var(--dp-ink-3)] py-10 text-center">
                No stack trace captured for this occurrence.
              </p>
            </div>

            <!-- ── Breadcrumbs tab ──────────────────────────────────────────── -->
            <div v-else-if="activeTab === 'breadcrumbs' && selectedEvent" class="space-y-5">
              <div v-if="dbCrumbs(selectedEvent).length" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-5">
                <p class="text-[11px] font-semibold text-[var(--dp-ink-2)] mb-3 flex items-center justify-between">
                  <span class="flex items-center gap-2">
                    <Database :size="13" class="text-emerald-600 shrink-0" />
                    Queries
                  </span>
                  <span class="text-[var(--dp-ink-3)] font-normal text-[10px]">
                    1–{{ dbCrumbs(selectedEvent).length }} of {{ dbCrumbs(selectedEvent).length }}
                  </span>
                </p>
                <div class="space-y-1">
                  <div v-for="(crumb, ci) in dbCrumbs(selectedEvent)" :key="ci"
                       class="dp-mono flex items-center gap-3 text-[11px] px-3 py-2 rounded-xl bg-[var(--dp-paper)] border border-[var(--dp-rule)]">
                    <Database :size="11" class="text-emerald-600 shrink-0" />
                    <span class="text-[var(--dp-ink-3)] shrink-0">{{ crumb.data?.connection ?? 'mysql' }}</span>
                    <span class="text-[var(--dp-ink-2)] flex-1 truncate">{{ crumb.message }}</span>
                    <span v-if="crumb.data?.duration_ms !== undefined"
                          class="text-[var(--dp-ink-3)] shrink-0">{{ crumb.data.duration_ms }}ms</span>
                  </div>
                </div>
              </div>

              <div v-if="nonDbCrumbs(selectedEvent).length" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-5">
                <p class="text-[11px] font-semibold text-[var(--dp-ink-2)] mb-3">Breadcrumbs</p>
                <div class="space-y-1 dp-mono text-[11px]">
                  <div v-for="(crumb, ci) in nonDbCrumbs(selectedEvent)" :key="ci"
                       class="flex items-start gap-2 text-[var(--dp-ink-3)]">
                    <span class="shrink-0 text-[var(--dp-ink-3)] tabular-nums">{{ crumb.timestamp ? crumb.timestamp.replace('T', ' ').slice(0, 19) : '' }}</span>
                    <span v-if="crumb.category" :class="crumbColor(crumb.level)" class="shrink-0">{{ crumb.category }}</span>
                    <span class="text-[var(--dp-ink-2)] truncate">{{ crumb.message }}</span>
                  </div>
                </div>
              </div>

              <p v-if="!dbCrumbs(selectedEvent).length && !nonDbCrumbs(selectedEvent).length"
                 class="text-[13px] text-[var(--dp-ink-3)] py-10 text-center">
                No breadcrumbs recorded for this occurrence.
              </p>
            </div>

            <!-- ── Request & Context tab ────────────────────────────────────── -->
            <div v-else-if="activeTab === 'request' && selectedEvent" class="space-y-5">
              <!-- Headers -->
              <div v-if="selectedEvent.payload?.request?.headers && Object.keys(selectedEvent.payload.request.headers).length"
                   class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl overflow-hidden">
                <details class="group">
                  <summary class="flex items-center justify-between px-5 py-3 text-[11px] font-semibold
                                  text-[var(--dp-ink-3)] cursor-pointer select-none list-none hover:text-[var(--dp-ink-2)] transition-colors">
                    <span>Headers</span>
                    <ChevronDown :size="14" class="transition-transform group-open:rotate-180" />
                  </summary>
                  <div class="px-5 pb-4">
                    <div v-for="(val, name) in selectedEvent.payload.request.headers" :key="name"
                         class="flex items-baseline gap-2 py-2 border-b border-[var(--dp-rule)] last:border-0">
                      <span class="dp-mono text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wider shrink-0 w-40 truncate">{{ name }}</span>
                      <span class="flex-1 border-b border-dotted border-[var(--dp-rule)] self-end mb-0.5 mx-1 min-w-0"></span>
                      <span class="dp-mono text-[11px] text-[var(--dp-ink-2)] break-all text-right max-w-xs truncate">{{ val }}</span>
                    </div>
                  </div>
                </details>
              </div>

              <!-- Body -->
              <div v-if="selectedEvent.payload?.request?.body" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl px-5 py-4">
                <p class="text-[11px] font-semibold text-[var(--dp-ink-3)] mb-3">Body</p>
                <pre class="dp-mono text-[12px] text-[var(--dp-ink-2)] bg-[var(--dp-paper)] rounded-xl px-4 py-3
                            overflow-x-auto border border-[var(--dp-rule)] leading-5 whitespace-pre-wrap">{{ fmt(selectedEvent.payload.request.body) }}</pre>
              </div>

              <!-- Routing -->
              <div v-if="selectedEvent.payload?.routing" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl px-5 py-4">
                <p class="text-[11px] font-semibold text-[var(--dp-ink-3)] mb-3">Routing</p>
                <div v-if="selectedEvent.payload.routing.controller"
                     class="flex items-baseline gap-2 py-2 border-b border-[var(--dp-rule)]">
                  <span class="dp-mono text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wider shrink-0 w-36">Controller</span>
                  <span class="flex-1 border-b border-dotted border-[var(--dp-rule)] self-end mb-0.5 mx-1"></span>
                  <span class="dp-mono text-[11px] text-[var(--dp-ink-2)] text-right break-all">{{ selectedEvent.payload.routing.controller }}</span>
                </div>
                <div v-if="selectedEvent.payload.routing.name"
                     class="flex items-baseline gap-2 py-2 border-b border-[var(--dp-rule)]">
                  <span class="dp-mono text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wider shrink-0 w-36">Route name</span>
                  <span class="flex-1 border-b border-dotted border-[var(--dp-rule)] self-end mb-0.5 mx-1"></span>
                  <span class="dp-mono text-[11px] text-[var(--dp-accent)] text-right">{{ selectedEvent.payload.routing.name }}</span>
                </div>
                <div v-if="selectedEvent.payload.routing.middleware?.length"
                     class="flex items-baseline gap-2 py-2 border-b border-[var(--dp-rule)]">
                  <span class="dp-mono text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wider shrink-0 w-36">Middleware</span>
                  <span class="flex-1 border-b border-dotted border-[var(--dp-rule)] self-end mb-0.5 mx-1"></span>
                  <span class="dp-mono text-[11px] text-[var(--dp-ink-2)] text-right">{{ selectedEvent.payload.routing.middleware.join(', ') }}</span>
                </div>
                <div class="mt-4">
                  <p class="dp-mono text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wider mb-2">Routing parameters</p>
                  <template v-if="Object.keys(selectedEvent.payload.routing.parameters ?? {}).length">
                    <div v-for="(val, key) in selectedEvent.payload.routing.parameters" :key="key"
                         class="flex items-baseline gap-2 py-1.5 border-b border-[var(--dp-rule)] last:border-0">
                      <span class="dp-mono text-[11px] text-[var(--dp-ink-3)] shrink-0 w-36 truncate">{{ key }}</span>
                      <span class="flex-1 border-b border-dotted border-[var(--dp-rule)] self-end mb-0.5 mx-1"></span>
                      <span class="dp-mono text-[11px] text-amber-700 text-right">{{ val }}</span>
                    </div>
                  </template>
                  <p v-else class="dp-mono text-[11px] text-[var(--dp-ink-3)]">// No routing parameters</p>
                </div>
              </div>

              <!-- Extra context -->
              <div v-if="selectedEvent.context && selectedEvent.context.platform !== 'wordpress' && !selectedEvent.context.request && !isVitalsEvent(selectedEvent)"
                   class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl overflow-hidden">
                <details class="group">
                  <summary class="flex items-center justify-between px-5 py-2.5 text-[11px] text-[var(--dp-ink-3)]
                                 hover:text-[var(--dp-ink-2)] cursor-pointer select-none list-none transition-colors">
                    <span>Extra context</span>
                    <ChevronDown :size="14" class="transition-transform group-open:rotate-180" />
                  </summary>
                  <div class="px-5 pb-4">
                    <pre class="dp-mono text-[11px] text-[var(--dp-ink-2)] bg-[var(--dp-paper)] rounded-xl p-3 overflow-x-auto border border-[var(--dp-rule)]">{{ fmt(selectedEvent.context) }}</pre>
                  </div>
                </details>
              </div>

              <p v-if="!hasRequestContent" class="text-[13px] text-[var(--dp-ink-3)] py-10 text-center">
                No request data captured for this occurrence.
              </p>
            </div>

            <!-- ── AI Analysis tab ──────────────────────────────────────────── -->
            <div v-else-if="activeTab === 'ai'">
              <div class="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-3">
                <h2 class="text-[11px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium flex items-center gap-1.5">
                  <span>Issue-level analysis</span>
                  <span v-if="ai?.cached" class="text-[9px] px-1.5 py-0.5 rounded bg-[var(--dp-surface-2)] text-[var(--dp-ink-3)] font-bold tracking-wider">cached</span>
                </h2>

                <div class="flex items-center gap-2 sm:ml-auto">
                  <Select v-model="selectedModel">
                    <SelectTrigger class="dp-select-trigger">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent class="bg-[var(--dp-surface)] border-[var(--dp-rule)] text-[var(--dp-ink)]">
                      <SelectItem value="auto" class="dp-select-item">Auto (smart selection)</SelectItem>
                      <SelectGroup>
                        <SelectLabel class="text-[var(--dp-ink-3)]">Anthropic</SelectLabel>
                        <SelectItem value="haiku"  :disabled="!providers.anthropic" class="dp-select-item">Claude Haiku 4.5{{ !providers.anthropic ? ' (not configured)' : '' }}</SelectItem>
                        <SelectItem value="sonnet" :disabled="!providers.anthropic" class="dp-select-item">Claude Sonnet 4.6{{ !providers.anthropic ? ' (not configured)' : '' }}</SelectItem>
                        <SelectItem value="opus"   :disabled="!providers.anthropic" class="dp-select-item">Claude Opus 4.6{{ !providers.anthropic ? ' (not configured)' : '' }}</SelectItem>
                      </SelectGroup>
                      <SelectGroup>
                        <SelectLabel class="text-[var(--dp-ink-3)]">OpenAI</SelectLabel>
                        <SelectItem value="gpt-4o-mini" :disabled="!providers.openai" class="dp-select-item">GPT-4o mini{{ !providers.openai ? ' (not configured)' : '' }}</SelectItem>
                        <SelectItem value="gpt-4o"      :disabled="!providers.openai" class="dp-select-item">GPT-4o{{ !providers.openai ? ' (not configured)' : '' }}</SelectItem>
                      </SelectGroup>
                      <SelectGroup>
                        <SelectLabel class="text-[var(--dp-ink-3)]">Google</SelectLabel>
                        <SelectItem value="gemini-flash" :disabled="!providers.gemini" class="dp-select-item">Gemini 2.0 Flash{{ !providers.gemini ? ' (not configured)' : '' }}</SelectItem>
                        <SelectItem value="gemini-pro"   :disabled="!providers.gemini" class="dp-select-item">Gemini 1.5 Pro{{ !providers.gemini ? ' (not configured)' : '' }}</SelectItem>
                      </SelectGroup>
                    </SelectContent>
                  </Select>

                  <button
                    @click="runAnalysis"
                    :disabled="aiLoading"
                    class="flex items-center gap-1.5 whitespace-nowrap shrink-0 text-[11px] px-3 py-1.5 rounded-xl font-medium transition-all
                           bg-[var(--dp-accent-soft)] text-[var(--dp-accent)] hover:opacity-80 disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    <Loader2 v-if="aiLoading" :size="12" class="animate-spin" />
                    <Sparkles v-else :size="12" />
                    {{ aiLoading ? 'Analysing…' : ai ? 'Re-analyse' : 'Analyse with AI' }}
                  </button>
                </div>
              </div>

              <div v-if="aiError" class="bg-red-500/10 border border-red-500/20 rounded-xl px-4 py-3 text-sm text-red-700">
                {{ aiError }}
              </div>

              <div v-else-if="ai" class="space-y-3">
                <div class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
                  <div class="flex items-start gap-3">
                    <span :class="severityBadge(ai.severity)"
                          class="shrink-0 text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wider mt-0.5">
                      {{ ai.severity }}
                    </span>
                    <p class="text-[13px] text-[var(--dp-ink)] font-medium leading-snug">{{ ai.root_cause }}</p>
                  </div>
                </div>
                <div class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
                  <p class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium mb-1.5">Explanation</p>
                  <p class="text-[13px] text-[var(--dp-ink-2)] leading-relaxed">{{ ai.explanation }}</p>
                </div>
                <div class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
                  <p class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium mb-1.5">How to fix</p>
                  <p class="text-[13px] text-[var(--dp-ink-2)] leading-relaxed">{{ ai.fix_suggestion }}</p>
                </div>
                <div v-if="ai.code_example" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl overflow-hidden">
                  <p class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium px-4 pt-3 pb-2">Code example</p>
                  <pre class="dp-mono text-[12px] text-[var(--dp-ink-2)] bg-[var(--dp-paper)] px-4 pb-4 overflow-x-auto leading-5 whitespace-pre-wrap">{{ ai.code_example }}</pre>
                </div>
                <div v-if="ai.prevention" class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
                  <p class="text-[10px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium mb-1.5">Prevention</p>
                  <p class="text-[13px] text-[var(--dp-ink-2)] leading-relaxed">{{ ai.prevention }}</p>
                </div>

                <div class="flex items-center justify-between flex-wrap gap-2">
                  <div v-if="ai.model_auto && ai.model_reason"
                    class="flex items-center gap-1.5 text-[10px] text-[var(--dp-ink-3)]">
                    <Info :size="11" />
                    <span class="italic">{{ ai.model_reason }}</span>
                  </div>
                  <p class="text-[10px] text-[var(--dp-ink-3)] ml-auto">
                    <span :class="modelBadgeColor(ai.model)" class="font-semibold">{{ ai.model }}</span>
                    · {{ ai.model_auto ? 'auto-selected' : 'manual' }}
                    · {{ ai.cached ? 'cached' : 'fresh' }}
                  </p>
                </div>
              </div>

              <div v-else class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl px-4 py-6 text-center">
                <p class="text-[13px] text-[var(--dp-ink-2)]">
                  Click <span class="text-[var(--dp-accent)] font-medium">Analyse with AI</span> to get root cause,
                  fix suggestions, and prevention tips.
                </p>
                <p class="text-[11px] text-[var(--dp-ink-3)] mt-1">
                  Model: <span class="text-[var(--dp-ink-2)]">{{ selectedModel === 'auto' ? 'auto-selected based on issue complexity' : selectedModel }}</span>
                </p>
              </div>
            </div>
            <p v-else class="text-[13px] text-[var(--dp-ink-3)] py-10 text-center">
              No occurrences recorded for this issue yet.
            </p>

          </div>

          <!-- ── Right: occurrences + fingerprint ──────────────────────────── -->
          <div class="space-y-5">
            <div class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl overflow-hidden">
              <p class="text-[11px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium px-4 pt-4 pb-2">
                Occurrences · {{ issue.events?.length ?? 0 }}
              </p>
              <div v-if="issue.events?.length" class="divide-y divide-[var(--dp-rule)] max-h-80 overflow-y-auto">
                <button v-for="(ev, idx) in issue.events" :key="ev.id" type="button"
                  @click="selectEvent(idx)"
                  class="w-full flex items-center justify-between gap-2 px-4 py-2.5 text-left transition-colors hover:bg-[var(--dp-surface-2)]/60"
                  :class="idx === selectedEventIdx ? 'bg-[var(--dp-accent-soft)]/60' : ''">
                  <span class="flex items-center gap-2 min-w-0">
                    <span class="w-1.5 h-1.5 rounded-full shrink-0" :class="idx === selectedEventIdx ? 'bg-[var(--dp-accent)]' : 'bg-[var(--dp-rule)]'"></span>
                    <span class="text-[12px] truncate" :class="idx === selectedEventIdx ? 'text-[var(--dp-ink)] font-medium' : 'text-[var(--dp-ink-2)]'">
                      {{ occurrenceLabel(idx) }}
                    </span>
                    <span v-if="ev.environment" :class="envBadge(ev.environment)"
                          class="text-[9px] px-1.5 py-0.5 rounded font-bold uppercase tracking-wide shrink-0">
                      {{ ev.environment }}
                    </span>
                  </span>
                  <span class="dp-mono text-[10px] text-[var(--dp-ink-3)] tabular-nums shrink-0">{{ timeAgo(ev.created_at) }}</span>
                </button>
              </div>
              <p v-else class="px-4 pb-4 text-[12px] text-[var(--dp-ink-3)]">No occurrences recorded.</p>
            </div>

            <div class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl p-4">
              <p class="text-[11px] text-[var(--dp-ink-3)] uppercase tracking-wide font-medium mb-2 flex items-center gap-1.5">
                <Fingerprint :size="12" />
                Fingerprint
              </p>
              <code class="dp-mono text-[12px] text-[var(--dp-ink-2)] break-all">{{ issue.fingerprint }}</code>
            </div>
          </div>

        </div>

      </template>

      <!-- Not found -->
      <div v-else class="flex flex-col items-center justify-center py-24 text-center">
        <div class="text-3xl mb-3">🔍</div>
        <p class="text-[var(--dp-ink)] font-medium">Issue not found</p>
      </div>

    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import axios from 'axios'
import {
  ArrowLeft, Check, X, User, AlertTriangle, ChevronDown, Plus, Minus,
  Loader2, Sparkles, FileText, Code2, History, Globe, Database, Package,
  Gauge, Info, Fingerprint,
} from 'lucide-vue-next'
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@/components/ui/breadcrumb'
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@/components/ui/select'

const route                = useRoute()
const router               = useRouter()
const issue                = ref(null)
const loading              = ref(true)
const ai                   = ref(null)
const aiLoading            = ref(false)
const aiError              = ref(null)

// ── Occurrence selection + tabs ───────────────────────────────────────────────
const selectedEventIdx = ref(0)
const activeTab        = ref('details')
const selectedEvent    = computed(() => issue.value?.events?.[selectedEventIdx.value] ?? null)

const TABS = [
  { value: 'details',     label: 'Details',           icon: FileText },
  { value: 'trace',       label: 'Stack Trace',       icon: Code2 },
  { value: 'breadcrumbs', label: 'Breadcrumbs',       icon: History },
  { value: 'request',     label: 'Request & Context', icon: Globe },
  { value: 'ai',          label: 'AI Analysis',       icon: Sparkles },
]

function occurrenceLabel(idx) {
  return idx === 0 ? 'Latest' : `Occurrence #${idx + 1}`
}

function eventHasDetails(e) {
  if (!e) return false
  return isVitalsEvent(e) || !!e.payload?.is_fatal || !!e.payload?.command
    || !!e.payload?.user || e.context?.platform === 'wordpress'
}

function eventHasRequest(e) {
  if (!e) return false
  const hasHeaders = e.payload?.request?.headers && Object.keys(e.payload.request.headers).length
  const hasExtra   = e.context && e.context.platform !== 'wordpress' && !e.context.request && !isVitalsEvent(e)
  return !!hasHeaders || !!e.payload?.request?.body || !!e.payload?.routing || !!hasExtra
}

// Land on whichever tab actually has content for this occurrence — a plain
// exception (the common case) has nothing under Details, so defaulting there
// hid the stack trace behind an extra click.
function bestTabFor(e) {
  if (!e) return 'details'
  if (e.payload?.exception?.stacktrace?.length) return 'trace'
  if (eventHasDetails(e)) return 'details'
  if (dbCrumbs(e).length || nonDbCrumbs(e).length) return 'breadcrumbs'
  if (eventHasRequest(e)) return 'request'
  return 'details'
}

function selectEvent(idx) {
  selectedEventIdx.value = idx
  if (activeTab.value !== 'ai') activeTab.value = bestTabFor(issue.value?.events?.[idx])
}

const hasDetailsContent = computed(() => eventHasDetails(selectedEvent.value))
const hasRequestContent = computed(() => eventHasRequest(selectedEvent.value))

// ── AI model selection ────────────────────────────────────────────────────────
const selectedModel = ref('auto')
const providers     = ref({ anthropic: false, openai: false, gemini: false })

const modelBadgeColor = (model) => {
  if (!model) return 'text-[var(--dp-ink-3)]'
  const m = model.toLowerCase()
  if (m.includes('haiku'))  return 'text-sky-700'
  if (m.includes('sonnet')) return 'text-[var(--dp-accent)]'
  if (m.includes('opus'))   return 'text-amber-700'
  if (m.includes('gpt'))    return 'text-emerald-700'
  if (m.includes('gemini')) return 'text-blue-700'
  return 'text-[var(--dp-ink-2)]'
}
const showAssignee         = ref(false)
const assigneeInput        = ref('')
const expandedPlugins      = ref(new Set())
const collapsedFrames      = ref(new Set())
const expandedVendorGroups = ref(new Set())

function togglePlugins(eventId) {
  const s = new Set(expandedPlugins.value)
  if (s.has(eventId)) s.delete(eventId)
  else s.add(eventId)
  expandedPlugins.value = s
}

// Collapse/expand individual code snippets (open by default)
function toggleFrame(eventIdx, groupIdx) {
  const key = `${eventIdx}-${groupIdx}`
  const s = new Set(collapsedFrames.value)
  if (s.has(key)) s.delete(key)
  else s.add(key)
  collapsedFrames.value = s
}

function toggleVendorGroup(eventIdx, groupIdx) {
  const key = `${eventIdx}-${groupIdx}`
  const s = new Set(expandedVendorGroups.value)
  if (s.has(key)) s.delete(key)
  else s.add(key)
  expandedVendorGroups.value = s
}

// ── Performance Vitals ────────────────────────────────────────────────────

function isVitalsEvent(event) {
  return event.payload?.message === 'Performance vitals' && !!event.context?.vitals
}

// Thresholds based on Google Lighthouse / Web Vitals standards
const VITAL_META = {
  lcp:       { label: 'LCP',       unit: 'ms',  good: 2500,  poor: 4000  },
  ttfb:      { label: 'TTFB',      unit: 'ms',  good: 800,   poor: 1800  },
  inp:       { label: 'INP',       unit: 'ms',  good: 200,   poor: 500   },
  page_load: { label: 'Load',      unit: 'ms',  good: 3000,  poor: 6000  },
  cls:       { label: 'CLS',       unit: '',    good: 0.1,   poor: 0.25  },
}

function vitalScore(key, value) {
  const m = VITAL_META[key]
  if (!m) return 50
  // Linear interpolation: good threshold → 90, poor threshold → 10
  if (value <= m.good) return Math.round(90 + (1 - value / m.good) * 10)
  if (value >= m.poor) return Math.max(0, Math.round(10 - (value - m.poor) / m.poor * 10))
  const ratio = (value - m.good) / (m.poor - m.good)
  return Math.round(90 - ratio * 80)
}

function vitalColor(score) {
  if (score >= 90) return '#059669' // emerald-600 — legible on the light card surface
  if (score >= 50) return '#d97706' // amber-600
  return '#dc2626'                  // red-600
}

function getVitals(event) {
  const v = event.context?.vitals ?? {}
  return Object.entries(VITAL_META)
    .filter(([key, meta]) => {
      const val = v[key]
      if (val === undefined) return false
      // Skip zero-value ms metrics — 0 means the measurement wasn't captured
      if (meta.unit === 'ms' && val <= 0) return false
      return true
    })
    .map(([key, meta]) => {
      const value = v[key]
      const score = vitalScore(key, value)
      return {
        key,
        label: meta.label,
        raw:   meta.unit === 'ms' ? `${value}ms` : value.toFixed(3),
        score,
        color: vitalColor(score),
      }
    })
}

function isVendorFrame(frame) {
  const file = (frame.file ?? '').replace(/\\/g, '/')
  return file.includes('/vendor/') || file.includes('\\vendor\\')
    || frame.plugin?.type === 'core'
}

function pluginBadgeColor(type) {
  return ({ plugin: 'bg-indigo-500/10 text-indigo-700', 'mu-plugin': 'bg-violet-500/10 text-violet-700',
            theme: 'bg-teal-500/10 text-teal-700' })[type] ?? 'bg-gray-500/10 text-gray-600'
}

function pluginBadgeLabel(plugin) {
  if (!plugin) return ''
  const prefix = ({ plugin: 'PLUGIN', 'mu-plugin': 'MU-PLUGIN', theme: 'THEME', core: 'WP' })[plugin.type] ?? ''
  return plugin.name ? `${prefix} ${plugin.name}` : prefix
}

function groupFrames(frames) {
  const groups = []
  let vendorBatch = []
  for (const frame of frames) {
    if (isVendorFrame(frame)) {
      vendorBatch.push(frame)
    } else {
      if (vendorBatch.length) {
        groups.push({ type: 'vendor', frames: vendorBatch })
        vendorBatch = []
      }
      groups.push({ type: 'app', frame })
    }
  }
  if (vendorBatch.length) groups.push({ type: 'vendor', frames: vendorBatch })
  return groups
}

function dbCrumbs(event) {
  return (event.breadcrumbs ?? []).filter(c => c.category === 'db' || c.type === 'query')
}

function nonDbCrumbs(event) {
  return (event.breadcrumbs ?? []).filter(c => c.category !== 'db' && c.type !== 'query')
}

function shortPath(file) {
  if (!file) return ''
  return file.replace(/\\/g, '/').split('/').slice(-2).join('/')
}

onMounted(async () => {
  try {
    const [issueRes] = await Promise.all([
      axios.get(`/api/issues/${route.params.id}`),
      axios.get(`/api/issues/${route.params.id}/analyze`)
        .then(r => { ai.value = r.data })
        .catch(() => {}),
      axios.get('/api/ai/providers')
        .then(r => { providers.value = r.data })
        .catch(() => {}),
    ])
    issue.value = issueRes.data
    activeTab.value = bestTabFor(issue.value?.events?.[0])
  } finally {
    loading.value = false
  }
})

async function runAnalysis() {
  aiLoading.value = true
  aiError.value   = null
  try {
    const { data } = await axios.post(
      `/api/issues/${route.params.id}/analyze`,
      { model: selectedModel.value },
    )
    ai.value = data
  } catch (err) {
    if (err.response?.status === 429) {
      aiError.value = 'Analysis was run recently. Please wait a moment before re-analysing.'
    } else {
      aiError.value = err.response?.data?.error ?? 'Analysis failed. Check that an AI provider key is configured.'
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
  ({ GET: 'bg-emerald-500/10 text-emerald-700', POST: 'bg-blue-500/10 text-blue-700',
     PUT: 'bg-amber-500/10 text-amber-700', PATCH: 'bg-amber-500/10 text-amber-700',
     DELETE: 'bg-red-500/10 text-red-700' })[m] ?? 'bg-gray-500/10 text-gray-600'

function formatMemory(bytes) {
  if (!bytes) return ''
  if (bytes >= 1073741824) return (bytes / 1073741824).toFixed(1) + ' GB'
  if (bytes >= 1048576)    return (bytes / 1048576).toFixed(0) + ' MB'
  return (bytes / 1024).toFixed(0) + ' KB'
}

const priorityBadge = (p) =>
  ({ critical: 'bg-red-500/15 text-red-700', high: 'bg-orange-500/15 text-orange-700',
     medium: 'bg-amber-500/15 text-amber-700', low: 'bg-blue-500/15 text-blue-700' })[p]
  ?? 'bg-gray-500/10 text-gray-600'

const crumbColor = (level) =>
  ({ error: 'text-red-700', warning: 'text-amber-700', info: 'text-blue-700', debug: 'text-[var(--dp-ink-3)]' })[level]
  ?? 'text-[var(--dp-ink-3)]'

const severityBadge = (s) =>
  ({ critical: 'bg-red-500/15 text-red-700', high: 'bg-orange-500/15 text-orange-700',
     medium: 'bg-amber-500/15 text-amber-700', low: 'bg-blue-500/15 text-blue-700' })[s]
  ?? 'bg-gray-500/10 text-gray-600'

const levelBadge = (level) =>
  ({ error: 'bg-red-500/10 text-red-700', warning: 'bg-amber-500/10 text-amber-700', info: 'bg-blue-500/10 text-blue-700' })[level]
  ?? 'bg-gray-500/10 text-gray-600'

const statusColor = (s) =>
  ({ unresolved: 'text-red-600', resolved: 'text-emerald-600', ignored: 'text-[var(--dp-ink-2)]' })[s] ?? 'text-[var(--dp-ink-2)]'

const envBadge = (env) =>
  ({ production: 'bg-red-500/10 text-red-700', staging: 'bg-amber-500/10 text-amber-700',
     development: 'bg-emerald-500/10 text-emerald-700' })[env?.toLowerCase()]
  ?? 'bg-gray-500/10 text-gray-600'

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

<style scoped>
/* Hallmark · macrostructure: dense technical readout, full layout rethink
 * genre: modern-minimal (clean/formal) · theme: DevPulse locked system (design.md)
 * body: Inter · outlier(mono): JetBrains Mono for every technical value
 * Restructured from "render every occurrence in full" to a Sentry-style
 * occurrence-switcher: one selected event's payload feeds a tabbed reader
 * (Details / Stack Trace / Breadcrumbs / Request & Context / AI Analysis),
 * with a right-rail occurrence list + fingerprint — same boxed rounded-2xl
 * language as every other page, see design.md § Macrostructure family.
 */
.issue-detail-root {
  background: var(--dp-paper);
  font-family: var(--dp-font-body);
  color: var(--dp-ink);
}

.dp-mono {
  font-family: var(--dp-font-mono);
  font-variant-numeric: tabular-nums;
}

/* shadcn Select styled to match the app's other filter/select fields */
.dp-select-trigger {
  min-width: 11rem;
  background: var(--dp-paper);
  border-color: var(--dp-rule);
  border-radius: 0.75rem;
  padding: 0.5rem 0.75rem;
  height: auto;
  font-size: 0.75rem;
  color: var(--dp-ink);
}

.dp-select-item:focus {
  background: var(--dp-accent-soft);
  color: var(--dp-accent);
}
</style>
