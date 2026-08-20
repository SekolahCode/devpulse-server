# Design — DevPulse Dashboard

A locked design system for this app. Every page redesign reads this file before
emitting code. Do not regenerate per page — extend or amend this file when the
system needs to grow.

## Genre

modern-minimal, dialed toward **luxury/formal** — restraint over density,
generous whitespace, hairline rules over boxed panels, quiet typography over
loud badges. This is the second full pass on the system (see Migration
history): the violet/instrument-panel register is retired in favour of an
exclusive, market-ready white register aimed at a paying enterprise audience,
not just an internal engineering console.

## App shell

N3 side-rail (`src/App.vue`): white rail (w-56, sticky, full-height,
`--dp-surface` on hairline `--dp-rule` border) with a serif wordmark, Projects
+ Analytics nav, live-WS status, sign out. Active nav state is a **quiet left
tick** (2px accent bar + ink text) rather than a filled pill — a filled active
pill reads casual/startup; a tick reads like a ledger index. Collapses to a
light top bar below `lg`.

## Macrostructure family

Layout changed app-wide, not just re-skinned. Every page-type gets a distinct,
named shape — still grouped by page type, per the multi-page discipline:

- **App pages (Projects, Issues) — Ledger Row.** Dropped the bordered/boxed
  console panel. Rows sit directly on the page paper, separated only by
  hairline dividers; the "panel" is implied by a top rule + quiet small-caps
  column heads, not a drawn box. Entry names set in the display serif at a
  larger size than before; figures stay right-aligned mono. Generous row
  height (py-5, was py-4) — a formal register needs room to breathe.
- **Analytics — Briefing.** Retired the instrument-panel stat grid in favour
  of a report-style read: one large serif-scale lead metric (the *number*
  itself stays mono per the numerals rule below, but it now sits inside an
  editorial header block, not a bordered instrument card), a quiet
  label:value strip for supporting stats (no boxed tiles), and charts framed
  as **exhibits** — hairline-bordered, captioned below in small text, no
  card shadow.
- **Releases — Dossier Timeline.** Timeline retained (it is structurally
  correct for release history) but reframed: thinner hairline spine, record
  cards with serif version headings, quieter single-line issue-stat chips
  instead of three loud colored pills.
- **IssueDetails — dense technical readout, exhibit-framed.** Still body +
  mono only — a stack trace has no use for a display face, and a formal tone
  doesn't mean airier code. Only the chrome around the trace changes: panels
  adopt the same hairline "exhibit" language as Analytics/Releases instead of
  boxed-card-with-shadow, for cross-page consistency.
- **Login — matches the shell.** Previously a standalone hardcoded dark
  screen; now built from the same tokens as everything else. First thing a
  buyer sees — it must not contradict "exclusive/formal."

## Theme

Anchor moved off violet (281°) to a warm near-neutral white paired with a
single **oxblood/wine accent** — reads formal and exclusive rather than
startup SaaS blue/violet. Tokens in `src/tokens.css`:

- `--dp-paper`        oklch(98.2% 0.004 55)  — page background, soft warm white
- `--dp-surface`      oklch(99.4% 0.002 55)  — card/row surface, brighter than paper
- `--dp-surface-2`    oklch(95%   0.008 55)  — wash: header rows, hover, zebra
- `--dp-ink`          oklch(20%   0.014 40)  — near-black warm ink
- `--dp-ink-2`        oklch(42%   0.010 40)
- `--dp-ink-3`        oklch(58%   0.008 40)
- `--dp-rule`         oklch(89%   0.006 55)  — hairline
- `--dp-accent`       oklch(33%   0.135 15)  — oxblood/wine
- `--dp-accent-hover` oklch(28%   0.13  15)
- `--dp-accent-soft`  oklch(94%   0.02  15)
- `--dp-danger`       oklch(51%   0.19  25)  — kept a true red, distinct from accent by
  lightness + chroma so "danger" never reads as "brand"
- `--dp-danger-soft`  oklch(96%   0.02  25)
- `--dp-ok`           oklch(48%   0.11  155) — muted forest, less candy than before

Elevated-dark surfaces (toast stack, chart tooltip) stay dark on purpose — an
elevated dark chip over a light page reads as intentional, not inconsistent —
but are now warm-neutral-tuned instead of violet-tuned:
`--dp-elevated-bg`, `--dp-elevated-border`, `--dp-elevated-title`,
`--dp-elevated-body`.

Chart.js can't consume `oklch()`, so `DashboardView.vue` keeps a small
hex-equivalent `C` constant mirroring these tokens for canvas rendering.
Semantic status colors (error/warning/info, priority, environment, plugin
type, HTTP method) are **unchanged** — they're functional, not brand, and
already read restrained (`-700` text on `/10` washes).

## Typography

- Display: **Fraunces** 500–600, roman only (`--dp-font-display`) — the
  single biggest lever for "formal/exclusive"; replaces Space Grotesk's
  techy-geometric voice with an editorial serif.
- Body: **Geist** 400–600 (`--dp-font-body`) — kept; already neutral and
  professional, no reason to churn it.
- Mono outlier: **JetBrains Mono** — kept; all numerals/counts/DSNs/chart
  ticks, `tabular-nums`. A lead metric or ledger figure is still always mono,
  even inside an otherwise-serif page — precision voice doesn't change.
- Loaded once via `src/tokens.css` Google Fonts import. Never import fonts
  per-view.

## Spacing

Tailwind 4-pt utilities, slightly more generous than before: page gutter
`px-6`, page block `py-10`–`py-12`, ledger row `py-5`, content column
`max-w-5xl`–`max-w-6xl mx-auto`.

## Motion

Motion-cut project — unchanged, and it already fit "formal": the single
sanctioned primitive is the number-reveal count-up on Analytics stats. No
scroll reveals, no hover lifts. `prefers-reduced-motion` collapses everything.
Easings: `--dp-ease-out` cubic-bezier(0.16, 1, 0.3, 1) · `--dp-ease-in`
cubic-bezier(0.7, 0, 0.84, 0).

## Microinteractions stance

- Silent success (copy button label swaps to "Copied ✓", no toast)
- Toasts for failures and hidden-effect actions only
- Focus: `:focus-visible` 2px `--dp-accent` outline, never animated
- Esc closes modals; click-outside dismisses

## CTA voice

- Primary: oxblood **`rounded-md`** fill (was `rounded-full` pill — a pill
  reads casual/startup; `rounded-md` reads formal). White text,
  `hover:--dp-accent-hover`, `active:translate-y-px`.
- Secondary: `--dp-surface-2` fill, `--dp-ink-2` text, `rounded-md`.
- Destructive: red-600 fill, only inside confirm dialogs.

## Data honesty

Counts shown on app pages come from real endpoints
(`/api/issues?project_id=…&limit=1` → `total`, `last_seen`). Never render
invented numbers; loading = pulse block, error = "—".

## What pages MUST share

- The white rail shell, serif wordmark, and oxblood accent (≤ 5% of viewport)
- The three-font system and mono-for-numerals rule
- CTA voice (`rounded-md`, not pill) and focus treatment
- Hairline-rule discipline over boxed/shadowed panels

## What pages MAY differ on

- Panel composition (ledger row vs briefing vs dossier timeline vs technical
  readout)
- Whether a display face is used at all (IssueDetails skips it — see above)

## Migration history

- Violet/instrument-panel light system — 2026-07-04 (first light pages,
  established `--dp-*` tokens, full app migrated off dark)
- **Formal/exclusive white re-theme + full layout pass — 2026-08-21**
  (this pass): retired violet 281° for an oxblood/wine anchor, Space Grotesk
  → Fraunces for display, `rounded-full` CTAs → `rounded-md`, boxed
  console/instrument panels → hairline ledger/briefing/dossier/exhibit
  layouts across all five views + shell + Login (Login rebuilt from a
  standalone hardcoded dark screen onto the shared system for the first
  time).

## Exports

### tokens.css
Canonical copy lives at `src/tokens.css` (imported first in `src/style.css`).

### Tailwind v4 @theme
```css
@theme {
  --color-dp-paper:   oklch(98.2% 0.004 55);
  --color-dp-ink:     oklch(20%   0.014 40);
  --color-dp-accent:  oklch(33%   0.135 15);
  --font-dp-display:  "Fraunces", serif;
  --font-dp-body:     "Geist", sans-serif;
  --font-dp-mono:     "JetBrains Mono", monospace;
}
```

### DTCG tokens.json
```json
{
  "color": {
    "paper":  { "$value": "oklch(98.2% 0.004 55)", "$type": "color" },
    "ink":    { "$value": "oklch(20% 0.014 40)",    "$type": "color" },
    "accent": { "$value": "oklch(33% 0.135 15)",    "$type": "color" }
  },
  "font": {
    "display": { "$value": "Fraunces",       "$type": "fontFamily" },
    "body":    { "$value": "Geist",          "$type": "fontFamily" },
    "mono":    { "$value": "JetBrains Mono", "$type": "fontFamily" }
  }
}
```

### shadcn/ui CSS variables
The project's existing shadcn tokens in `src/style.css` `:root` are a dark
set (`--background`, `--card`, `--popover`, `--muted`, `--border`, etc.),
historically shared by every page. `--primary`, `--ring`, `--chart-1`,
`--sidebar-primary`, and `--sidebar-ring` were updated in this pass to the
new oxblood accent (`oklch(0.33 0.135 15)`) — they'd been left at violet-600
and were leaking into the shadcn `<Skeleton>` component's default
`bg-primary/10`, the one shadcn primitive that doesn't get a per-usage
color override anywhere it's used. The rest of the dark set stays
unchanged: Analytics (the only page using shadcn `<Card>`/`<Select>`)
overrides those per-usage (`bg-[var(--dp-surface)]`, etc.) rather than
changing the globals. Override locally with `--dp-*` per-usage if a future
page adopts these components, and check any *other* shadcn primitive
that's added later for the same unstyled-default leak `<Skeleton>` had.
