-- AI analyses cache — one row per issue, overwritten on re-analysis
CREATE TABLE IF NOT EXISTS ai_analyses (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    issue_id      UUID        NOT NULL REFERENCES issues(id) ON DELETE CASCADE,
    root_cause    TEXT        NOT NULL,
    explanation   TEXT        NOT NULL,
    fix_suggestion TEXT       NOT NULL,
    code_example  TEXT,
    severity      TEXT        NOT NULL DEFAULT 'medium',
    prevention    TEXT,
    model         TEXT        NOT NULL DEFAULT 'claude-sonnet-4-6',
    analyzed_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT ai_analyses_issue_unique UNIQUE (issue_id)
);

CREATE INDEX IF NOT EXISTS idx_ai_analyses_issue_id ON ai_analyses(issue_id);

-- Add affected_users count to issues (incremented via worker)
ALTER TABLE issues ADD COLUMN IF NOT EXISTS affected_users INTEGER NOT NULL DEFAULT 0;
