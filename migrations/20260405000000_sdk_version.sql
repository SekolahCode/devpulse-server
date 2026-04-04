-- Track which SDK (name + version) sent each event.
-- Allows identifying users on old SDK versions with known bugs.
ALTER TABLE events ADD COLUMN IF NOT EXISTS sdk_version TEXT;
