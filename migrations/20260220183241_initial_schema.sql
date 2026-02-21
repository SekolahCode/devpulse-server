-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE projects (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       VARCHAR(255) NOT NULL,
    api_key    VARCHAR(64)  UNIQUE NOT NULL,
    platform   VARCHAR(50),
    created_at TIMESTAMPTZ  DEFAULT NOW()
);

CREATE TABLE issues (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id  UUID REFERENCES projects(id) ON DELETE CASCADE,
    fingerprint VARCHAR(64) NOT NULL,
    title       TEXT NOT NULL,
    level       VARCHAR(20) DEFAULT 'error',
    status      VARCHAR(20) DEFAULT 'unresolved',
    first_seen  TIMESTAMPTZ DEFAULT NOW(),
    last_seen   TIMESTAMPTZ DEFAULT NOW(),
    event_count INT DEFAULT 1,
    UNIQUE(project_id, fingerprint)
);

CREATE TABLE events (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    issue_id   UUID REFERENCES issues(id) ON DELETE CASCADE,
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    payload    JSONB NOT NULL,
    context    JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE alerts (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    channel    VARCHAR(50),
    endpoint   TEXT,
    enabled    BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
