-- Add model selection metadata to ai_analyses
ALTER TABLE ai_analyses
    ADD COLUMN IF NOT EXISTS model_auto   BOOLEAN DEFAULT TRUE  NOT NULL,
    ADD COLUMN IF NOT EXISTS model_reason TEXT    DEFAULT NULL;
