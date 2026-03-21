-- ── Stats query indexes ───────────────────────────────────────────────────────
-- Supports COUNT(*) FILTER (WHERE status = '...' AND last_seen > NOW() - INTERVAL '24 hours')
CREATE INDEX IF NOT EXISTS idx_issues_status_last_seen ON issues(status, last_seen DESC);

-- Supports the regressions_24h filter: first_seen + last_seen + status
CREATE INDEX IF NOT EXISTS idx_issues_status_first_last ON issues(status, first_seen, last_seen DESC);

-- Supports global events count: COUNT(*) WHERE created_at > NOW() - INTERVAL '24 hours'
-- (the existing idx_events_project_created requires a project_id match)
CREATE INDEX IF NOT EXISTS idx_events_created ON events(created_at DESC);
