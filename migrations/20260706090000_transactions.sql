-- Named performance transactions (duration only, no spans/tracing) — separate
-- from the issues/events pipeline: no fingerprinting, no alerts, high volume.
CREATE TABLE IF NOT EXISTS transactions (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id   UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name         TEXT NOT NULL,
    duration_ms  DOUBLE PRECISION NOT NULL,
    release      TEXT,
    environment  TEXT NOT NULL DEFAULT 'production',
    context      JSONB,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_transactions_project_created ON transactions(project_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_transactions_project_name    ON transactions(project_id, name);
