-- Source maps for deobfuscating minified JS stack traces.
-- Matched at read time against `(project_id, release, artifact_name)`, where
-- artifact_name is the exact `file` string as it appears in a stack frame.
CREATE TABLE IF NOT EXISTS source_maps (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id    UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    release       TEXT NOT NULL,
    artifact_name TEXT NOT NULL,
    source_map    JSONB NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(project_id, release, artifact_name)
);

CREATE INDEX IF NOT EXISTS idx_source_maps_lookup
    ON source_maps(project_id, release, artifact_name);
