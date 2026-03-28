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
        <span :class="priorityBadge(issue.priority)"
              class="text-[10px] font-bold px-2 py-0.5 rounded-md uppercase tracking-wide">
          {{ issue.priority ?? 'medium' }} priority
        </span>
        <span v-if="issue.last_release" class="text-[11px] text-gray-400 bg-white/5 px-2 py-0.5 rounded font-mono">
          v{{ issue.last_release }}
        </span>
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

      <!-- ── Recent occurrences ─────────────────────────────────────────── -->
      <div v-if="issue.events?.length" class="mb-6">
        <h2 class="text-[11px] text-gray-500 uppercase tracking-wide font-medium mb-3">
          Recent occurrences
        </h2>
        <div class="space-y-6">
          <div v-for="(event, idx) in issue.events" :key="event.id"
               class="bg-[#0e0e18] border border-white/8 rounded-xl overflow-hidden">

            <!-- ── Exception header ──────────────────────────────────────── -->
            <div v-if="event.payload?.exception" class="px-6 pt-6 pb-5 border-b border-white/6">
              <!-- Short type -->
              <h3 class="text-2xl font-bold text-white mb-1 font-mono">
                {{ event.payload.exception.type?.split('\\').pop() ?? 'Error' }}
              </h3>
              <!-- Throw-site file:line -->
              <p v-if="event.payload.exception.stacktrace?.[0]?.file"
                 class="text-[12px] text-gray-500 font-mono mb-3">
                {{ event.payload.exception.stacktrace[0].file }}:{{ event.payload.exception.stacktrace[0].line }}
              </p>
              <!-- Full message -->
              <p class="text-[14px] text-gray-300 leading-relaxed mb-4">
                {{ event.payload.exception.message }}
              </p>
              <!-- Badges -->
              <div class="flex flex-wrap items-center gap-2">
                <span v-if="event.context?.laravel"
                      class="text-[10px] font-bold px-2 py-0.5 rounded bg-purple-600/20 text-purple-300 font-mono tracking-wide">
                  LARAVEL {{ event.context.laravel }}
                </span>
                <span v-if="event.context?.php"
                      class="text-[10px] font-bold px-2 py-0.5 rounded bg-blue-500/20 text-blue-300 font-mono tracking-wide">
                  PHP {{ event.context.php }}
                </span>
                <span class="text-[10px] font-bold px-2 py-0.5 rounded bg-amber-500/20 text-amber-400 tracking-wide">
                  ⚠ UNHANDLED
                </span>
                <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-white/5 text-gray-500 tracking-wide">
                  CODE {{ event.payload.exception.code ?? 0 }}
                </span>
                <span v-if="event.payload?.request?.method"
                      :class="methodColor(event.payload.request.method)"
                      class="text-[10px] font-bold px-2 py-0.5 rounded font-mono tracking-wide">
                  {{ event.payload.request.method }}
                </span>
                <span v-if="event.payload?.request?.url"
                      class="text-[11px] text-gray-500 font-mono truncate max-w-xs">
                  {{ event.payload.request.url }}
                </span>
                <!-- WordPress plugin attribution badge -->
                <span v-if="event.payload?.plugin"
                      :class="pluginBadgeColor(event.payload.plugin.type)"
                      class="text-[10px] font-bold px-2 py-0.5 rounded font-mono tracking-wide">
                  {{ pluginBadgeLabel(event.payload.plugin) }}
                </span>
              </div>
            </div>

            <!-- ── Non-exception event header ────────────────────────────── -->
            <div v-else class="flex items-center justify-between px-4 py-2.5 bg-[#0d0d16] border-b border-white/5">
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
                <span class="text-[11px] text-gray-600 tabular-nums">{{ formatDate(event.created_at) }}</span>
              </div>
            </div>

            <!-- ── Performance Vitals card (Lighthouse style) ───────────── -->
            <div v-if="isVitalsEvent(event)" class="px-6 py-5 border-b border-white/5">
              <p class="text-[11px] font-semibold text-gray-400 mb-5 flex items-center gap-2">
                <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-violet-400 shrink-0">
                  <path d="M12 2a10 10 0 1 0 10 10"/><path d="M12 6v6l4 2"/><path d="M22 2l-5 5"/><path d="M17 2h5v5"/>
                </svg>
                Performance Vitals
              </p>
              <div class="grid grid-cols-3 sm:grid-cols-5 gap-4">
                <div v-for="m in getVitals(event)" :key="m.key" class="flex flex-col items-center gap-2.5">
                  <!-- Score ring -->
                  <div class="relative w-15 h-15">
                    <svg viewBox="0 0 36 36" class="w-full h-full -rotate-90">
                      <!-- track -->
                      <circle cx="18" cy="18" r="15.9" fill="none" stroke="currentColor"
                              stroke-width="2.2" class="text-white/6"/>
                      <!-- arc: circumference ≈ 100, so dasharray = score/100 * 100 -->
                      <circle cx="18" cy="18" r="15.9" fill="none"
                              stroke-width="2.5" stroke-linecap="round"
                              :stroke="m.color"
                              :stroke-dasharray="`${m.score} 100`"/>
                    </svg>
                    <!-- score number -->
                    <span class="absolute inset-0 flex items-center justify-center text-[15px] font-bold"
                          :style="{ color: m.color }">
                      {{ m.score }}
                    </span>
                  </div>
                  <!-- label + raw value -->
                  <div class="text-center">
                    <p class="text-[11px] font-semibold text-gray-300 leading-tight">{{ m.label }}</p>
                    <p class="text-[10px] text-gray-600 tabular-nums mt-0.5">{{ m.raw }} / 100</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- ── Exception trace ────────────────────────────────────────── -->
            <div v-if="event.payload?.exception?.stacktrace?.length" class="px-5 py-5">
              <p class="text-[11px] font-semibold text-gray-400 mb-3 flex items-center gap-2">
                <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-amber-500 shrink-0">
                  <path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z"/>
                  <line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/>
                </svg>
                Exception trace
              </p>
              <div class="space-y-1.5">
                <template v-for="(group, gi) in groupFrames(event.payload.exception.stacktrace)">

                  <!-- App frame card -->
                  <div v-if="group.type === 'app'"
                       :key="`app-${gi}`"
                       class="border border-white/8 rounded-lg overflow-hidden">
                    <!-- Frame header -->
                    <div class="flex items-center justify-between gap-3 px-4 py-2.5 bg-[#0d0d16]">
                      <div class="flex items-center gap-2.5 min-w-0 flex-1">
                        <span class="shrink-0 w-2 h-2 rounded-full"
                              :class="gi === 0 ? 'bg-red-400' : 'bg-gray-600'"></span>
                        <span class="text-[12px] font-mono truncate"
                              :class="gi === 0 ? 'text-gray-200' : 'text-gray-400'">
                          <span v-if="group.frame.function" class="text-violet-400">{{ group.frame.function }}</span>
                          <span v-else-if="group.frame.file" class="text-gray-500">{{ group.frame.file.split('/').pop() }}</span>
                          <span v-else class="text-gray-700 italic">unknown</span>
                        </span>
                      </div>
                      <div class="flex items-center gap-2 shrink-0">
                        <span v-if="group.frame.plugin && group.frame.plugin.type !== 'core'"
                              :class="pluginBadgeColor(group.frame.plugin.type)"
                              class="text-[10px] font-mono px-1.5 py-0.5 rounded">
                          {{ group.frame.plugin.name }}
                        </span>
                        <span class="text-[11px] text-gray-600 font-mono">
                          {{ shortPath(group.frame.file) }}:{{ group.frame.line }}
                        </span>
                        <button v-if="group.frame.context"
                                @click="toggleFrame(idx, gi)"
                                class="w-5 h-5 flex items-center justify-center rounded border border-white/10
                                       hover:border-white/25 text-gray-600 hover:text-gray-300 transition-colors text-[11px] shrink-0">
                          {{ collapsedFrames.has(`${idx}-${gi}`) ? '+' : '×' }}
                        </button>
                      </div>
                    </div>
                    <!-- Code snippet — open by default, collapse with × -->
                    <div v-if="group.frame.context && !collapsedFrames.has(`${idx}-${gi}`)">
                      <div v-for="(codeLine, lineNum) in group.frame.context.lines"
                           :key="lineNum"
                           class="flex items-stretch font-mono text-[12px] leading-6"
                           :class="Number(lineNum) === group.frame.line ? 'bg-red-500/10' : 'hover:bg-white/[0.015]'">
                        <span class="shrink-0 w-12 text-right pr-3 select-none tabular-nums py-0.5 border-r"
                              :class="Number(lineNum) === group.frame.line
                                ? 'text-red-400 font-bold border-red-500/50 bg-red-500/15'
                                : 'text-gray-700 border-white/5'">
                          {{ lineNum }}
                        </span>
                        <span class="pl-4 py-0.5 whitespace-pre overflow-x-auto flex-1"
                              :class="Number(lineNum) === group.frame.line ? 'text-red-100' : 'text-gray-400'">{{ codeLine }}</span>
                      </div>
                    </div>
                  </div>

                  <!-- Vendor frames group -->
                  <div v-else :key="`vendor-${gi}`" class="border border-white/5 rounded-lg overflow-hidden">
                    <button @click="toggleVendorGroup(idx, gi)"
                            class="w-full flex items-center gap-2.5 px-4 py-2 text-[11px] text-gray-600
                                   hover:text-gray-400 hover:bg-white/3 transition-colors">
                      <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="shrink-0">
                        <path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/>
                      </svg>
                      <span>{{ group.frames.length }} vendor frame{{ group.frames.length !== 1 ? 's' : '' }}</span>
                      <span class="ml-auto transition-transform duration-150"
                            :class="expandedVendorGroups.has(`${idx}-${gi}`) ? 'rotate-180' : ''">▾</span>
                    </button>
                    <div v-if="expandedVendorGroups.has(`${idx}-${gi}`)" class="border-t border-white/5">
                      <div v-for="(vf, vfi) in group.frames" :key="vfi"
                           class="flex items-start gap-3 px-4 py-1.5 border-b border-white/3 last:border-0 hover:bg-white/2">
                        <span class="shrink-0 w-1.5 h-1.5 rounded-full bg-gray-700 mt-1.5"></span>
                        <span class="text-[11px] text-gray-600 font-mono flex-1 truncate">
                          {{ vf.function ?? (vf.file?.split('/').pop() ?? 'unknown') }}
                        </span>
                        <span class="text-[10px] text-gray-700 font-mono shrink-0">
                          {{ shortPath(vf.file) }}{{ vf.line ? `:${vf.line}` : '' }}
                        </span>
                      </div>
                    </div>
                  </div>

                </template>
              </div>
            </div>

            <!-- ── Queries (from DB breadcrumbs) ─────────────────────────── -->
            <div v-if="dbCrumbs(event).length" class="px-5 py-4 border-t border-white/5">
              <p class="text-[11px] font-semibold text-gray-400 mb-3 flex items-center justify-between">
                <span class="flex items-center gap-2">
                  <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-emerald-500 shrink-0">
                    <ellipse cx="12" cy="5" rx="9" ry="3"/>
                    <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
                    <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
                  </svg>
                  Queries
                </span>
                <span class="text-gray-700 font-normal text-[10px]">
                  1–{{ dbCrumbs(event).length }} of {{ dbCrumbs(event).length }}
                </span>
              </p>
              <div class="space-y-1">
                <div v-for="(crumb, ci) in dbCrumbs(event)" :key="ci"
                     class="flex items-center gap-3 font-mono text-[11px] px-3 py-2 rounded-lg bg-[#090912] border border-white/5">
                  <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-emerald-600 shrink-0">
                    <ellipse cx="12" cy="5" rx="9" ry="3"/>
                    <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
                    <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
                  </svg>
                  <span class="text-gray-600 shrink-0">{{ crumb.data?.connection ?? 'mysql' }}</span>
                  <span class="text-gray-300 flex-1 truncate">{{ crumb.message }}</span>
                  <span v-if="crumb.data?.duration_ms !== undefined"
                        class="text-gray-600 shrink-0">{{ crumb.data.duration_ms }}ms</span>
                </div>
              </div>
            </div>

            <!-- ── Non-DB breadcrumbs ─────────────────────────────────────── -->
            <div v-if="nonDbCrumbs(event).length" class="px-5 py-4 border-t border-white/5">
              <p class="text-[11px] font-semibold text-gray-400 mb-3">Breadcrumbs</p>
              <div class="space-y-1 font-mono text-[11px]">
                <div v-for="(crumb, ci) in nonDbCrumbs(event)" :key="ci"
                     class="flex items-start gap-2 text-gray-500">
                  <span class="shrink-0 text-gray-700 tabular-nums">{{ crumb.timestamp ? crumb.timestamp.replace('T', ' ').slice(0, 19) : '' }}</span>
                  <span v-if="crumb.category" :class="crumbColor(crumb.level)" class="shrink-0">{{ crumb.category }}</span>
                  <span class="text-gray-400 truncate">{{ crumb.message }}</span>
                </div>
              </div>
            </div>

            <!-- ── Headers ────────────────────────────────────────────────── -->
            <div v-if="event.payload?.request?.headers && Object.keys(event.payload.request.headers).length"
                 class="border-t border-white/5">
              <details class="group">
                <summary class="flex items-center justify-between px-5 py-3 text-[11px] font-semibold
                                text-gray-500 cursor-pointer select-none list-none hover:text-gray-400 transition-colors">
                  <span>Headers</span>
                  <span class="transition-transform group-open:rotate-180">▾</span>
                </summary>
                <div class="px-5 pb-4">
                  <div v-for="(val, name) in event.payload.request.headers" :key="name"
                       class="flex items-baseline gap-2 py-2 border-b border-white/4 last:border-0">
                    <span class="text-[10px] font-mono text-gray-600 uppercase tracking-wider shrink-0 w-40 truncate">{{ name }}</span>
                    <span class="flex-1 border-b border-dotted border-white/8 self-end mb-0.5 mx-1 min-w-0"></span>
                    <span class="text-[11px] font-mono text-gray-400 break-all text-right max-w-xs truncate">{{ val }}</span>
                  </div>
                </div>
              </details>
            </div>

            <!-- ── Body ───────────────────────────────────────────────────── -->
            <div v-if="event.payload?.request?.body" class="border-t border-white/5 px-5 py-4">
              <p class="text-[11px] font-semibold text-gray-500 mb-3">Body</p>
              <pre class="text-[12px] text-gray-300 font-mono bg-[#090912] rounded-lg px-4 py-3
                          overflow-x-auto border border-white/5 leading-5 whitespace-pre-wrap">{{ fmt(event.payload.request.body) }}</pre>
            </div>

            <!-- ── Routing ─────────────────────────────────────────────────── -->
            <div v-if="event.payload?.routing" class="border-t border-white/5 px-5 py-4">
              <p class="text-[11px] font-semibold text-gray-500 mb-3">Routing</p>
              <div v-if="event.payload.routing.controller"
                   class="flex items-baseline gap-2 py-2 border-b border-white/4">
                <span class="text-[10px] font-mono text-gray-600 uppercase tracking-wider shrink-0 w-36">Controller</span>
                <span class="flex-1 border-b border-dotted border-white/8 self-end mb-0.5 mx-1"></span>
                <span class="text-[11px] font-mono text-gray-300 text-right break-all">{{ event.payload.routing.controller }}</span>
              </div>
              <div v-if="event.payload.routing.name"
                   class="flex items-baseline gap-2 py-2 border-b border-white/4">
                <span class="text-[10px] font-mono text-gray-600 uppercase tracking-wider shrink-0 w-36">Route name</span>
                <span class="flex-1 border-b border-dotted border-white/8 self-end mb-0.5 mx-1"></span>
                <span class="text-[11px] font-mono text-violet-400 text-right">{{ event.payload.routing.name }}</span>
              </div>
              <div v-if="event.payload.routing.middleware?.length"
                   class="flex items-baseline gap-2 py-2 border-b border-white/4">
                <span class="text-[10px] font-mono text-gray-600 uppercase tracking-wider shrink-0 w-36">Middleware</span>
                <span class="flex-1 border-b border-dotted border-white/8 self-end mb-0.5 mx-1"></span>
                <span class="text-[11px] font-mono text-gray-400 text-right">{{ event.payload.routing.middleware.join(', ') }}</span>
              </div>
              <!-- Routing parameters -->
              <div class="mt-4">
                <p class="text-[10px] font-mono text-gray-600 uppercase tracking-wider mb-2">Routing parameters</p>
                <template v-if="Object.keys(event.payload.routing.parameters ?? {}).length">
                  <div v-for="(val, key) in event.payload.routing.parameters" :key="key"
                       class="flex items-baseline gap-2 py-1.5 border-b border-white/4 last:border-0">
                    <span class="text-[11px] font-mono text-gray-600 shrink-0 w-36 truncate">{{ key }}</span>
                    <span class="flex-1 border-b border-dotted border-white/8 self-end mb-0.5 mx-1"></span>
                    <span class="text-[11px] font-mono text-amber-400 text-right">{{ val }}</span>
                  </div>
                </template>
                <p v-else class="text-[11px] font-mono text-gray-700">// No routing parameters</p>
              </div>
            </div>

            <!-- ── Fatal error notice ─────────────────────────────────────── -->
            <div v-if="event.payload?.is_fatal" class="px-5 py-4 border-t border-white/5">
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

            <!-- ── Laravel command context ────────────────────────────────── -->
            <div v-if="event.payload?.command" class="px-5 py-4 border-t border-white/5">
              <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-2">Artisan Command</p>
              <div class="flex flex-wrap items-center gap-2">
                <span class="text-[12px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-1 rounded">
                  php artisan {{ event.payload.command }}
                </span>
                <span v-if="event.payload.exit_code !== undefined"
                      class="text-[11px] font-mono text-red-400 bg-red-500/10 px-2 py-0.5 rounded">
                  exit {{ event.payload.exit_code }}
                </span>
                <span v-if="event.payload.input?.trim()" class="text-[11px] text-gray-400 font-mono">
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

            <!-- ── User ───────────────────────────────────────────────────── -->
            <div v-if="event.payload?.user" class="px-5 py-4 border-t border-white/5">
              <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-2">User</p>
              <div class="flex flex-wrap gap-2">
                <span v-if="event.payload.user.id"       class="text-[11px] text-gray-400 bg-white/5 px-2 py-0.5 rounded font-mono">ID: {{ event.payload.user.id }}</span>
                <span v-if="event.payload.user.email"    class="text-[11px] text-gray-400 bg-white/5 px-2 py-0.5 rounded">{{ event.payload.user.email }}</span>
                <span v-if="event.payload.user.username" class="text-[11px] text-gray-400 bg-white/5 px-2 py-0.5 rounded">{{ event.payload.user.username }}</span>
              </div>
            </div>

            <!-- ── WordPress platform metadata ────────────────────────────── -->
            <div v-if="event.context?.platform === 'wordpress'" class="px-5 py-4 border-t border-white/5">
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
                <span v-if="event.context.is_admin"     class="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/15 text-amber-400 font-semibold">admin</span>
                <span v-if="event.context.multisite"    class="text-[10px] px-1.5 py-0.5 rounded bg-blue-500/15 text-blue-400 font-semibold">multisite</span>
                <span v-if="event.context.wp_debug"     class="text-[10px] px-1.5 py-0.5 rounded bg-orange-500/15 text-orange-400 font-semibold">WP_DEBUG</span>
                <span v-if="event.context.wp_debug_log" class="text-[10px] px-1.5 py-0.5 rounded bg-orange-500/15 text-orange-400 font-semibold">DEBUG_LOG</span>
              </div>
            </div>

            <!-- ── Active plugins ─────────────────────────────────────────── -->
            <div v-if="event.context?.active_plugins?.length" class="px-5 py-4 border-t border-white/5">
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
                   class="font-mono text-[11px] text-gray-500 space-y-0.5 max-h-36 overflow-y-auto">
                <div v-for="p in event.context.active_plugins" :key="p"
                     class="truncate hover:text-gray-400 transition-colors">{{ p }}</div>
              </div>
              <div v-else class="text-[11px] text-gray-700 italic">Click to expand</div>
            </div>

            <!-- ── Extra context (non-WP, non-routing) ────────────────────── -->
            <div v-if="event.context && event.context.platform !== 'wordpress' && !event.context.request"
                 class="border-t border-white/5">
              <details class="group">
                <summary class="flex items-center justify-between px-5 py-2.5 text-[11px] text-gray-600
                               hover:text-gray-400 cursor-pointer select-none list-none transition-colors">
                  <span>Extra context</span>
                  <span class="transition-transform group-open:rotate-180 inline-block">▾</span>
                </summary>
                <div class="px-5 pb-4">
                  <pre class="text-[11px] text-gray-400 bg-[#090912] rounded-lg p-3 overflow-x-auto">{{ fmt(event.context) }}</pre>
                </div>
              </details>
            </div>

          </div>
        </div>
      </div>

      <!-- ── AI Analysis panel ──────────────────────────────────────────── -->
      <div class="mb-6">
        <div class="flex items-center justify-between mb-3 gap-3 flex-wrap">
          <h2 class="text-[11px] text-gray-500 uppercase tracking-wide font-medium flex items-center gap-1.5">
            <span>AI Analysis</span>
            <span v-if="ai?.cached" class="text-[9px] px-1.5 py-0.5 rounded bg-gray-700/60 text-gray-500 font-bold tracking-wider">cached</span>
          </h2>

          <div class="flex items-center gap-2 ml-auto">
            <!-- Model picker -->
            <div class="flex items-center bg-[#111119] border border-white/6 rounded-lg p-0.5 gap-0.5">
              <button
                v-for="m in MODEL_OPTIONS" :key="m.value"
                @click="selectedModel = m.value"
                :class="selectedModel === m.value
                  ? 'bg-[#1e1e30] text-white'
                  : 'text-gray-500 hover:text-gray-300'"
                class="px-2.5 py-1 rounded-md text-[10px] font-medium transition-all whitespace-nowrap"
                :title="m.description"
              >
                {{ m.label }}
              </button>
            </div>

            <!-- Analyse button -->
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
        </div>

        <div v-if="aiError" class="bg-red-500/10 border border-red-500/20 rounded-xl px-4 py-3 text-sm text-red-400">
          {{ aiError }}
        </div>

        <div v-else-if="ai" class="space-y-3">
          <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
            <div class="flex items-start gap-3">
              <span :class="severityBadge(ai.severity)"
                    class="shrink-0 text-[10px] font-bold px-2 py-0.5 rounded-md uppercase tracking-wider mt-0.5">
                {{ ai.severity }}
              </span>
              <p class="text-[13px] text-gray-200 font-medium leading-snug">{{ ai.root_cause }}</p>
            </div>
          </div>
          <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
            <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-1.5">Explanation</p>
            <p class="text-[13px] text-gray-300 leading-relaxed">{{ ai.explanation }}</p>
          </div>
          <div class="bg-[#111119] border border-white/6 rounded-xl p-4">
            <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-1.5">How to fix</p>
            <p class="text-[13px] text-gray-300 leading-relaxed">{{ ai.fix_suggestion }}</p>
          </div>
          <div v-if="ai.code_example" class="bg-[#111119] border border-white/6 rounded-xl overflow-hidden">
            <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium px-4 pt-3 pb-2">Code example</p>
            <pre class="text-[12px] text-gray-300 font-mono bg-[#0a0a10] px-4 pb-4 overflow-x-auto leading-5 whitespace-pre-wrap">{{ ai.code_example }}</pre>
          </div>
          <div v-if="ai.prevention" class="bg-[#111119] border border-white/6 rounded-xl p-4">
            <p class="text-[10px] text-gray-600 uppercase tracking-wide font-medium mb-1.5">Prevention</p>
            <p class="text-[13px] text-gray-400 leading-relaxed">{{ ai.prevention }}</p>
          </div>

          <!-- Model footer -->
          <div class="flex items-center justify-between flex-wrap gap-2">
            <div v-if="ai.model_auto && ai.model_reason"
              class="flex items-center gap-1.5 text-[10px] text-gray-600">
              <svg width="10" height="10" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8">
                <circle cx="8" cy="8" r="6"/><line x1="8" y1="5" x2="8" y2="8"/><circle cx="8" cy="11" r="0.5" fill="currentColor"/>
              </svg>
              <span class="italic">{{ ai.model_reason }}</span>
            </div>
            <p class="text-[10px] text-gray-700 ml-auto">
              <span :class="modelBadgeColor(ai.model)" class="font-semibold">{{ ai.model }}</span>
              · {{ ai.model_auto ? 'auto-selected' : 'manual' }}
              · {{ ai.cached ? 'cached' : 'fresh' }}
            </p>
          </div>
        </div>

        <div v-else class="bg-[#111119] border border-white/6 rounded-xl px-4 py-6 text-center">
          <p class="text-[13px] text-gray-500">
            Click <span class="text-violet-400 font-medium">Analyse with AI</span> to get root cause,
            fix suggestions, and prevention tips.
          </p>
          <p class="text-[11px] text-gray-700 mt-1">
            Model: <span class="text-gray-500">{{ selectedModel === 'auto' ? 'auto-selected based on issue complexity' : selectedModelLabel }}</span>
          </p>
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
import { computed, ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import axios from 'axios'

const route                = useRoute()
const router               = useRouter()
const issue                = ref(null)
const loading              = ref(true)
const ai                   = ref(null)
const aiLoading            = ref(false)
const aiError              = ref(null)

// ── AI model selection ────────────────────────────────────────────────────────
const MODEL_OPTIONS = [
  { value: 'auto',   label: 'Auto',   description: 'Automatically pick the best model based on issue complexity' },
  { value: 'haiku',  label: 'Haiku',  description: 'Claude Haiku 4.5 — fast, great for simple/high-frequency errors' },
  { value: 'sonnet', label: 'Sonnet', description: 'Claude Sonnet 4.6 — balanced accuracy and speed (recommended)' },
  { value: 'opus',   label: 'Opus',   description: 'Claude Opus 4.6 — most capable, best for deep/complex issues' },
]
const selectedModel      = ref('auto')
const selectedModelLabel = computed(() =>
  MODEL_OPTIONS.find(m => m.value === selectedModel.value)?.description ?? ''
)

const modelBadgeColor = (model) => {
  if (!model) return 'text-gray-500'
  const m = model.toLowerCase()
  if (m.includes('haiku'))  return 'text-sky-400'
  if (m.includes('sonnet')) return 'text-violet-400'
  if (m.includes('opus'))   return 'text-amber-400'
  return 'text-gray-400'
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
  if (score >= 90) return '#34d399' // emerald-400
  if (score >= 50) return '#fbbf24' // amber-400
  return '#f87171'                  // red-400
}

function getVitals(event) {
  const v = event.context?.vitals ?? {}
  return Object.entries(VITAL_META)
    .filter(([key]) => v[key] !== undefined)
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
  return ({ plugin: 'bg-indigo-500/20 text-indigo-300', 'mu-plugin': 'bg-violet-500/20 text-violet-300',
            theme: 'bg-teal-500/20 text-teal-300' })[type] ?? 'bg-gray-700/50 text-gray-400'
}

function pluginBadgeLabel(plugin) {
  if (!plugin) return ''
  const prefix = ({ plugin: '🧩', 'mu-plugin': '🔌', theme: '🎨', core: 'WP' })[plugin.type] ?? ''
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
    const { data } = await axios.post(
      `/api/issues/${route.params.id}/analyze`,
      { model: selectedModel.value },
    )
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
