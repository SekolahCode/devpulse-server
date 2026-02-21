#!/usr/bin/env bash
# =============================================================================
# sqlx-prepare.sh
# Generates the .sqlx/ offline query cache required to build the Docker image.
#
# Run this once before your first `docker compose up -d --build`, and again
# any time you add or modify a sqlx::query! / sqlx::query_as! call.
#
# Requirements:
#   - Docker (already installed if you're running this)
#   - sqlx-cli: cargo install sqlx-cli --no-default-features --features postgres
# =============================================================================
set -euo pipefail

# ── Config ────────────────────────────────────────────────────────────────────
PG_CONTAINER="devpulse-sqlx-prep-$$"
PG_PORT=15432
PG_PASS="sqlx_prep_secret"
DB_URL="postgres://postgres:${PG_PASS}@127.0.0.1:${PG_PORT}/devpulse"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# ── Check sqlx-cli is installed ───────────────────────────────────────────────
if ! command -v sqlx &>/dev/null && ! cargo sqlx --version &>/dev/null 2>&1; then
  echo "❌  sqlx-cli not found. Install it with:"
  echo ""
  echo "    cargo install sqlx-cli --no-default-features --features postgres"
  echo ""
  exit 1
fi

# ── Cleanup on exit (even on error) ──────────────────────────────────────────
cleanup() {
  echo "▶  Removing temporary postgres container..."
  docker rm -f "$PG_CONTAINER" >/dev/null 2>&1 || true
}
trap cleanup EXIT

# ── Start temporary postgres ──────────────────────────────────────────────────
echo "▶  Starting temporary postgres on port ${PG_PORT}..."
docker run -d \
  --name "$PG_CONTAINER" \
  -e POSTGRES_PASSWORD="$PG_PASS" \
  -e POSTGRES_DB=devpulse \
  -p "127.0.0.1:${PG_PORT}:5432" \
  postgres:16-alpine >/dev/null

# ── Wait for postgres to be ready ────────────────────────────────────────────
echo -n "▶  Waiting for postgres"
for i in $(seq 1 30); do
  if docker exec "$PG_CONTAINER" pg_isready -U postgres -d devpulse &>/dev/null; then
    echo " ready."
    break
  fi
  echo -n "."
  sleep 1
  if [[ $i -eq 30 ]]; then
    echo ""
    echo "❌  Timed out waiting for postgres."
    exit 1
  fi
done

# ── Run migrations so sqlx can introspect the schema ─────────────────────────
echo "▶  Running migrations..."
cd "$PROJECT_ROOT"
DATABASE_URL="$DB_URL" sqlx migrate run

# ── Generate the offline query cache ─────────────────────────────────────────
echo "▶  Running cargo sqlx prepare..."
DATABASE_URL="$DB_URL" cargo sqlx prepare

echo ""
echo "✅  Done! The .sqlx/ directory has been updated."
echo ""
echo "    Next steps:"
echo "    1. Commit the cache:"
echo "       git add .sqlx/"
echo "       git commit -m 'chore: update sqlx offline query cache'"
echo ""
echo "    2. Build and start:"
echo "       docker compose up -d --build"
