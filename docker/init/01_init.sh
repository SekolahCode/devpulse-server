#!/usr/bin/env bash
# Runs once on first container init (skipped if pg_data volume already has data)
# Reads APP_DB_PASSWORD from the environment — set it in your .env file.
set -euo pipefail

APP_DB_PASSWORD="${APP_DB_PASSWORD:-secret}"

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    -- Enable pgcrypto extension
    CREATE EXTENSION IF NOT EXISTS "pgcrypto";

    -- Create dedicated app role (idempotent)
    DO \$\$
    BEGIN
      IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'devpulse') THEN
        CREATE ROLE devpulse WITH LOGIN PASSWORD '${APP_DB_PASSWORD}';
      ELSE
        ALTER ROLE devpulse WITH PASSWORD '${APP_DB_PASSWORD}';
      END IF;
    END
    \$\$;

    -- Grant full access to the devpulse database
    GRANT ALL PRIVILEGES ON DATABASE devpulse TO devpulse;
EOSQL

# Grant schema access (must run connected to the target database)
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    GRANT ALL ON SCHEMA public TO devpulse;
EOSQL
