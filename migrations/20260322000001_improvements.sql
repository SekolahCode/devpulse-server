-- Faster alert lookups (used in fire_alerts hot path)
CREATE INDEX IF NOT EXISTS idx_alerts_project_enabled ON alerts(project_id, enabled);

-- Environment column on issues for fast env filtering
ALTER TABLE issues ADD COLUMN IF NOT EXISTS environment TEXT NOT NULL DEFAULT 'production';
CREATE INDEX IF NOT EXISTS idx_issues_environment ON issues(environment);
