-- ── Releases ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS releases (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id  UUID        NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    version     TEXT        NOT NULL,
    ref         TEXT,                  -- git SHA / tag
    url         TEXT,                  -- deploy URL / CI link
    deployed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(project_id, version)
);
CREATE INDEX IF NOT EXISTS idx_releases_project ON releases(project_id, deployed_at DESC);

-- Release on issues (first seen / last seen in which version)
ALTER TABLE issues ADD COLUMN IF NOT EXISTS first_release TEXT;
ALTER TABLE issues ADD COLUMN IF NOT EXISTS last_release  TEXT;

-- Release + breadcrumbs on events
ALTER TABLE events ADD COLUMN IF NOT EXISTS release     TEXT;
ALTER TABLE events ADD COLUMN IF NOT EXISTS breadcrumbs JSONB;

-- Issue assignee & priority
ALTER TABLE issues ADD COLUMN IF NOT EXISTS assignee TEXT;
ALTER TABLE issues ADD COLUMN IF NOT EXISTS priority TEXT NOT NULL DEFAULT 'medium';

-- ── Affected-users deduplication ──────────────────────────────────────────────
-- Tracks unique user keys per issue so affected_users is a true distinct count.
CREATE TABLE IF NOT EXISTS issue_users (
    issue_id UUID NOT NULL REFERENCES issues(id) ON DELETE CASCADE,
    user_key TEXT NOT NULL,
    PRIMARY KEY (issue_id, user_key)
);
CREATE INDEX IF NOT EXISTS idx_issue_users_issue ON issue_users(issue_id);

-- ── Alert retry tracking ─────────────────────────────────────────────────────
ALTER TABLE alerts ADD COLUMN IF NOT EXISTS retry_count INTEGER     NOT NULL DEFAULT 0;
ALTER TABLE alerts ADD COLUMN IF NOT EXISTS retry_after TIMESTAMPTZ;
ALTER TABLE alerts ADD COLUMN IF NOT EXISTS last_error  TEXT;
