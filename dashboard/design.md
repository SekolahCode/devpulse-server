# Design — DevPulse Dashboard

A locked design system for this app. Every page redesign reads this file before
emitting code. Do not regenerate per page — extend or amend this file when the
system needs to grow.

## Genre

modern-minimal, clean/formal enterprise app-shell — quiet neutrals, a single
confident accent, boxed rounded surfaces over hairline-on-paper rows. This is
the third full pass on the system (see Migration history): the oxblood/serif
"exclusive ledger" register is retired in favour of a clean slate/indigo
system, explicitly modeled on the **teleradiology** project's app-shell
(read-only design reference — never edited) as the reference DNA: its
`DashboardSidebar.vue`, `DataTable.vue`, and list-page pattern
(`Radiologists/Index.vue`, `Admin/ActivityLog/Index.vue`).

## App shell

Floating rounded rail (`src/App.vue`): a `rounded-2xl` sidebar inset from the
edge (`inset-y-4 left-4`), translucent white with `backdrop-blur`, hairline
border + soft shadow — not a flush full-height panel. Collapsible (`w-64` ↔
`w-20`, circular chevron toggle), auto-collapses below `lg`. Active nav state
is a **quiet filled pill** (`bg-[var(--dp-accent-soft)]` + accent text) rather
than a left tick — matches the reference's rounded, boxed language instead of
the previous ledger-index tick. Collapses to a light top bar below `lg`.

## Macrostructure family

Every page-type keeps its own shape, but all now converge on the reference's
**boxed rounded surface** language instead of bare hairline-on-paper rows:

- **App pages (Projects, Issues) — boxed table.** Rows live inside a single
  `rounded-2xl border border-[var(--dp-rule)]` container with a washed header
  row (`bg-[var(--dp-surface-2)]/60`) and `divide-y` hairlines between rows —
  matches the reference `DataTable.vue` exactly. Dropped the display/serif
  entry-name treatment; names are plain `font-medium` body text, same voice as
  everything else on the page.
- **Analytics — Bento-adjacent briefing.** Stat strip + charts framed as
  rounded-2xl hairline-bordered panels (was: hairline "exhibit" strips with no
  box) — a quiet wash header, no card shadow, numerals stay mono.
- **Releases — Dossier Timeline.** Timeline retained (structurally correct for
  release history); record cards now sit in `rounded-2xl` bordered panels
  instead of a bare hairline spine card, headings in plain semibold body type.
- **IssueDetails — dense technical readout.** Still body + mono only. Panels
  now use the same `rounded-2xl` bordered-box language as every other page for
  cross-page consistency (previously hairline "exhibit" strips).
- **Login — matches the shell, keeps the one display-font moment.** The only
  page that still sets `--dp-font-display` (Space Grotesk) on its headline —
  mirrors the reference, where auth pages alone get a display treatment and
  every in-app page stays body-only.

## Theme

Anchor moved off oxblood/wine (15°) to **slate neutrals + a confident indigo
accent** (256°) — the reference's exact hue family, ported into this app's
existing `--dp-*` token names so page markup didn't need literal-class
rewrites everywhere. Tokens in `src/tokens.css`:

- `--dp-paper`        oklch(98.5% 0.004 250)  — page background, cool near-white
- `--dp-surface`      oklch(100%  0     0)    — card/row surface, true white
- `--dp-surface-2`    oklch(96.5% 0.008 250)  — wash: header rows, hover, shell bg
- `--dp-ink`          oklch(22%   0.022 258)  — near-black cool ink (slate-900)
- `--dp-ink-2`        oklch(34%   0.018 257)  (slate-600)
- `--dp-ink-3`        oklch(48%   0.014 256)  (slate-500)
- `--dp-rule`         oklch(86%   0.012 252)  — hairline (slate-200)
- `--dp-accent`       oklch(58%   0.2   256)  — indigo-600
- `--dp-accent-hover` oklch(53%   0.2   256)  — indigo-500/600
- `--dp-accent-soft`  oklch(93%   0.03  256)  — indigo-50/100 wash (active nav, avatar chips)
- `--dp-danger`       oklch(58%   0.2   25)   — red-600, unchanged in spirit
- `--dp-danger-soft`  oklch(96%   0.03  25)
- `--dp-ok`           oklch(56%   0.14  158)  — emerald-600

Elevated-dark surfaces (toast stack, chart tooltip) stay dark on purpose — an
elevated dark chip over a light page reads as intentional, not inconsistent —
now slate-tuned instead of warm-neutral-tuned:
`--dp-elevated-bg`, `--dp-elevated-border`, `--dp-elevated-title`,
`--dp-elevated-body`.

Chart.js can't consume `oklch()`, so `DashboardView.vue` keeps a small
hex-equivalent `C` constant mirroring these tokens for canvas rendering.
Semantic status colors (error/warning/info, priority, environment, plugin
type, HTTP method) are **unchanged** — they're functional, not brand.

## Typography

- Body: **Inter** 400–700 (`--dp-font-body`) — the primary voice everywhere in
  the app shell, matching the reference's app-shell pages exactly (no serif,
  no display face on headings — `text-2xl font-semibold tracking-tight` is
  the whole recipe).
- Display: **Space Grotesk** 500–700 (`--dp-font-display`) — reserved for
  Login only, mirroring the reference's own split (auth screens get a display
  moment, in-app CRUD/list pages never do).
- Mono outlier: **JetBrains Mono** — kept; all numerals/counts/DSNs/chart
  ticks, `tabular-nums`.
- Loaded once via `src/tokens.css` Google Fonts import. Never import fonts
  per-view.

## Spacing & shape

Tailwind 4-pt utilities, page gutter `px-6`, page block `py-6`–`py-10`. Radius
scale bumped rounder across the board to match the reference: inputs/buttons
`rounded-xl` (was `rounded-md`), modals/boxed panels/tables `rounded-2xl` (was
`rounded-md`/`rounded-lg`), icon-only action buttons `rounded-lg`, avatars/
pills stay `rounded-full`.

## Motion

Motion-cut project — unchanged. The single sanctioned primitive is the
number-reveal count-up on Analytics stats, plus the sidebar's width/opacity
transition on collapse (matches the reference's own sidebar transition). No
scroll reveals, no hover lifts beyond simple color transitions.
Easings: `--dp-ease-out` cubic-bezier(0.16, 1, 0.3, 1) · `--dp-ease-in`
cubic-bezier(0.7, 0, 0.84, 0).

## Microinteractions stance

- Silent success (copy button label swaps to "Copied ✓", no toast)
- Toasts for failures and hidden-effect actions only
- Focus: `:focus-visible` 2px `--dp-accent` outline, never animated
- Esc closes modals; click-outside dismisses

## CTA voice

- Primary: indigo **`rounded-xl`** fill, white text, `hover:--dp-accent-hover`.
- Secondary: `--dp-surface-2` fill, `--dp-ink-2` text, `rounded-xl`.
- Destructive: red-600 fill, only inside confirm dialogs.

## Data honesty

Counts shown on app pages come from real endpoints
(`/api/issues?project_id=…&limit=1` → `total`, `last_seen`). Never render
invented numbers; loading = pulse block, error = "—".

## What pages MUST share

- The floating rounded sidebar shell and indigo accent
- Inter as the only in-app-shell font (mono for numerals); Space Grotesk
  confined to Login
- CTA voice (`rounded-xl` fill) and focus treatment
- Boxed `rounded-2xl` hairline-bordered surfaces over bare hairline-on-paper
  rows

## What pages MAY differ on

- Panel composition (boxed table vs briefing vs dossier timeline vs technical
  readout)
- Whether Login's display face applies (it never does elsewhere)

## Migration history

- Violet/instrument-panel light system — 2026-07-04 (first light pages,
  established `--dp-*` tokens, full app migrated off dark)
- Formal/exclusive white re-theme + full layout pass — 2026-08-21 (oxblood
  accent, Fraunces display, hairline ledger/briefing/dossier/exhibit layouts,
  Login rebuilt onto the shared system)
- **Slate/indigo re-theme, teleradiology reference — 2026-08-21** (this
  pass): retired oxblood 15° for an indigo 256° accent matching the
  teleradiology reference's Cobalt palette; Fraunces → Space Grotesk (now
  Login-only) with Inter as the app-shell's only body/heading voice; radius
  scale bumped `rounded-md/lg` → `rounded-xl/2xl`; App.vue rebuilt as a
  floating collapsible rounded sidebar (was a flush full-height rail); Ledger
  Row / hairline-exhibit panels boxed into `rounded-2xl` bordered surfaces
  across Projects, Issues, Dashboard, Releases, IssueDetails.

## Exports

### tokens.css
Canonical copy lives at `src/tokens.css` (imported first in `src/style.css`).

### Tailwind v4 @theme
```css
@theme {
  --color-dp-paper:   oklch(98.5% 0.004 250);
  --color-dp-ink:     oklch(22%   0.022 258);
  --color-dp-accent:  oklch(58%   0.2   256);
  --font-dp-display:  "Space Grotesk", sans-serif;
  --font-dp-body:     "Inter", sans-serif;
  --font-dp-mono:     "JetBrains Mono", monospace;
}
```

### DTCG tokens.json
```json
{
  "color": {
    "paper":  { "$value": "oklch(98.5% 0.004 250)", "$type": "color" },
    "ink":    { "$value": "oklch(22% 0.022 258)",    "$type": "color" },
    "accent": { "$value": "oklch(58% 0.2 256)",      "$type": "color" }
  },
  "font": {
    "display": { "$value": "Space Grotesk",  "$type": "fontFamily" },
    "body":    { "$value": "Inter",          "$type": "fontFamily" },
    "mono":    { "$value": "JetBrains Mono", "$type": "fontFamily" }
  }
}
```

### shadcn/ui CSS variables
The project's existing shadcn tokens in `src/style.css` `:root` are a dark
set (`--background`, `--card`, `--popover`, `--muted`, `--border`, etc.),
historically shared by every page. `--primary`, `--ring`, `--chart-1`,
`--sidebar-primary`, and `--sidebar-ring` are updated in this pass to the new
indigo accent (`oklch(0.58 0.2 256)`) so they stay in sync with `--dp-accent`
rather than drifting back to a retired brand hue. The rest of the dark set
stays unchanged: Analytics (the only page using shadcn `<Card>`/`<Select>`)
overrides those per-usage (`bg-[var(--dp-surface)]`, etc.) rather than
changing the globals. Override locally with `--dp-*` per-usage if a future
page adopts these components, and check any *other* shadcn primitive added
later for the same unstyled-default leak `<Skeleton>` had.
