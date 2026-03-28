// ── Shared constants ──────────────────────────────────────────────────────────

export const PLATFORMS    = ['laravel', 'wordpress', 'php']
export const ENVIRONMENTS = ['production', 'staging', 'development']
export const STATUSES     = ['unresolved', 'resolved', 'ignored']
export const CHANNELS     = ['webhook', 'telegram', 'email']
export const PRIORITIES   = ['critical', 'high', 'medium', 'low']

// ── Shared colour helpers ─────────────────────────────────────────────────────

export const levelBadge = (level) =>
  ({ error: 'bg-red-500/15 text-red-400', warning: 'bg-amber-500/15 text-amber-400', info: 'bg-blue-500/15 text-blue-400' })[level]
  ?? 'bg-gray-700/50 text-gray-400'

export const levelDot = (level) =>
  ({ error: 'bg-red-400', warning: 'bg-amber-400', info: 'bg-blue-400' })[level] ?? 'bg-gray-500'

export const priorityBadge = (p) =>
  ({ critical: 'bg-red-500/20 text-red-400', high: 'bg-orange-500/20 text-orange-400',
     medium: 'bg-amber-500/20 text-amber-400', low: 'bg-blue-500/20 text-blue-400' })[p]
  ?? 'bg-gray-700/50 text-gray-400'

export const envBadge = (env) =>
  ({ production: 'bg-red-500/15 text-red-400', staging: 'bg-amber-500/15 text-amber-400',
     development: 'bg-emerald-500/15 text-emerald-400' })[env?.toLowerCase()]
  ?? 'bg-gray-700/50 text-gray-400'

export const statusColor = (s) =>
  ({ unresolved: 'text-red-400', resolved: 'text-emerald-400', ignored: 'text-gray-400' })[s]
  ?? 'text-gray-300'

export const platformIcon  = (p) => ({ laravel: '🔴', wordpress: '🔵', php: '🟣' })[p] ?? '⬜'
export const platformColor = (p) =>
  ({ laravel: 'bg-red-500/10', wordpress: 'bg-blue-500/10', php: 'bg-purple-500/10' })[p]
  ?? 'bg-gray-500/10'
