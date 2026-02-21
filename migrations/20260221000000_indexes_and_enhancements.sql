-- ── Performance indexes ───────────────────────────────────────────────────────
CREATE INDEX IF NOT EXISTS idx_issues_project_status  ON issues(project_id, status);
CREATE INDEX IF NOT EXISTS idx_issues_last_seen        ON issues(last_seen DESC);
CREATE INDEX IF NOT EXISTS idx_issues_title_search     ON issues USING gin(to_tsvector('english', title));
CREATE INDEX IF NOT EXISTS idx_events_issue_created    ON events(issue_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_events_project_created  ON events(project_id, created_at DESC);

-- ── Environment support ───────────────────────────────────────────────────────
ALTER TABLE events
    ADD COLUMN IF NOT EXISTS environment VARCHAR(50) DEFAULT 'production';

-- ── Alert cooldown ────────────────────────────────────────────────────────────
ALTER TABLE alerts
    ADD COLUMN IF NOT EXISTS cooldown_minutes INT         DEFAULT 60;
ALTER TABLE alerts
    ADD COLUMN IF NOT EXISTS last_alerted_at  TIMESTAMPTZ DEFAULT NULL;
