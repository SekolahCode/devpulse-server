# =============================================================================
# DevPulse — Production Dockerfile
# Multi-stage build: Node (frontend) → cargo-chef (dep cache) → Builder → Runtime
# =============================================================================
#
# ⚠️  PREREQUISITE — Offline sqlx query cache
# Before building this image, run once against a live database:
#
#   DATABASE_URL=postgres://postgres:changeme@localhost:5432/devpulse \
#     cargo sqlx prepare
#
# Then commit the generated .sqlx/ directory.
# The build will fail without it because cargo cannot connect to a DB at build time.
# =============================================================================

# ── Stage 1: Build the Vue dashboard ─────────────────────────────────────────
FROM node:22-alpine AS frontend
WORKDIR /app/dashboard

# Cache npm dependencies as a separate layer
COPY dashboard/package*.json ./
RUN npm ci --prefer-offline

# Copy source and build
COPY dashboard/ ./
RUN npm run build
# Output lands at /app/web/dist (vite.config.js: outDir: "../web/dist")

# ── Stage 2: cargo-chef — prepare dependency recipe ──────────────────────────
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ── Stage 3: cargo-chef — cook (cache compiled dependencies) ─────────────────
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# This layer is re-used on subsequent builds as long as dependencies don't change
RUN cargo chef cook --release --recipe-path recipe.json

# Copy sources + the compiled frontend (required by include_dir! macro)
COPY . .
COPY --from=frontend /app/web/dist /app/web/dist

# Tell sqlx to use the committed .sqlx/ cache instead of a live database
ENV SQLX_OFFLINE=true

RUN cargo build --release
# Binary is at /app/target/release/devpulse-server

# ── Stage 4: Minimal runtime image ───────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

# Install only what's needed: CA certs (for HTTPS alerts) + curl (healthcheck)
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

# Non-root user for security
RUN useradd -m -u 1001 -s /sbin/nologin devpulse

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/devpulse-server /app/devpulse-server
RUN chown devpulse:devpulse /app/devpulse-server

USER devpulse

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=5s --start-period=15s --retries=3 \
    CMD curl -fsS http://localhost:8000/health || exit 1

ENTRYPOINT ["/app/devpulse-server"]
