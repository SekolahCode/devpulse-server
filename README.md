# devpulse-server

Rust + Axum ingestion server with an embedded Vue 3 dashboard for DevPulse.

## Tech Stack

| Layer     | Technology                        |
|-----------|-----------------------------------|
| Runtime   | Rust (Tokio async)                |
| Web       | Axum                              |
| Database  | PostgreSQL (SQLx)                 |
| Cache     | Redis (Deadpool)                  |
| Dashboard | Vue 3 + Vite (embedded in binary) |

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- Rust toolchain (only needed for local builds outside Docker)

## Quick Start

```bash
cp .env.example .env
# Set at minimum: ADMIN_TOKEN, POSTGRES_PASSWORD, APP_DB_PASSWORD
docker compose up -d
```

Open [http://localhost:8000](http://localhost:8000)

> **Production requirement:** The server refuses to start if `ADMIN_TOKEN` is blank
> unless `RUST_LOG` is set to `debug` or `trace` (dev mode). Always set a strong
> token before deploying.

`docker compose up` starts three containers:

| Container  | Image                                | Purpose                        |
|------------|--------------------------------------|--------------------------------|
| `devpulse` | `sekolahcode/devpulse-server:1`      | Rust app + embedded dashboard  |
| `postgres` | `postgres:16-alpine`                 | Primary database               |
| `redis`    | `redis:7-alpine`                     | Rate-limit counters and caching|

PostgreSQL is initialised on first boot by `docker/init/01_init.sh`, which creates the `devpulse` app role. Redis requires no setup. Neither service exposes ports to the host by default.

## Development

```bash
# Start infrastructure (Postgres, Redis) + Mailpit for email previews
docker compose -f docker-compose.dev.yaml up -d

# Run the server locally (ADMIN_TOKEN blank is allowed when RUST_LOG=debug)
RUST_LOG=debug cargo run

# Run the dashboard with HMR
cd dashboard && npm run dev
```

## Building Locally

The `docker-compose.yaml` uses the pre-built image by default. To build from source:

```yaml
# Replace the image: line in docker-compose.yaml with:
build:
  context: .
  dockerfile: Dockerfile
```

```bash
docker compose build
```

The Dockerfile compiles the Rust binary and embeds the Vue dashboard via `include_dir!`.

## API Routes

| Method | Path                               | Auth     | Description                    |
|--------|------------------------------------|----------|--------------------------------|
| GET    | `/health`                          | None     | Health check                   |
| GET    | `/metrics`                         | None     | Prometheus-format metrics      |
| POST   | `/api/ingest`                      | API Key  | Ingest an error event          |
| GET    | `/ws`                              | Token    | WebSocket live event stream    |
| GET    | `/api/projects`                    | Token    | List projects                  |
| POST   | `/api/projects`                    | Token    | Create a project               |
| PATCH  | `/api/projects/{id}`               | Token    | Update a project               |
| DELETE | `/api/projects/{id}`               | Token    | Delete a project               |
| POST   | `/api/projects/{id}/rotate-key`    | Token    | Rotate a project's API key     |
| GET    | `/api/projects/{id}/releases`      | Token    | List releases                  |
| POST   | `/api/projects/{id}/releases`      | Token    | Create a release               |
| DELETE | `/api/releases/{id}`               | Token    | Delete a release               |
| GET    | `/api/projects/{id}/alerts`        | Token    | List alert rules               |
| POST   | `/api/projects/{id}/alerts`        | Token    | Create an alert rule           |
| PATCH  | `/api/alerts/{id}`                 | Token    | Update an alert rule           |
| DELETE | `/api/alerts/{id}`                 | Token    | Delete an alert rule           |
| GET    | `/api/issues`                      | Token    | List issues                    |
| GET    | `/api/issues/{id}`                 | Token    | Get a single issue             |
| PATCH  | `/api/issues/{id}`                 | Token    | Update an issue                |
| DELETE | `/api/issues/{id}`                 | Token    | Delete an issue                |
| GET    | `/api/issues/{id}/analyze`         | Token    | Get cached AI analysis         |
| POST   | `/api/issues/{id}/analyze`         | Token    | Run AI analysis                |
| GET    | `/api/ai/providers`                | Token    | List available AI providers    |
| GET    | `/api/stats`                       | Token    | Aggregated statistics          |
| GET    | `/api/stats/chart`                 | Token    | Chart data (Chart.js format)   |

### Ingest authentication

The ingest endpoint reads the API key from the **`X-API-Key` request header**.

```http
POST /api/ingest HTTP/1.1
Host: devpulse.example.com
Content-Type: application/json
X-API-Key: <your-project-api-key>

{ "exception": { ... } }
```

Every response includes an **`X-Trace-Id`** header (UUID) for log correlation. Propagate your own by sending it as a request header.

### Protected route authentication

All `Token`-authenticated routes require:

```http
Authorization: Bearer <ADMIN_TOKEN>
```

### WebSocket authentication

The live event stream requires the same token as a query parameter:

```
ws://your-host/ws?token=<ADMIN_TOKEN>
```

## Environment Variables

| Variable                  | Default              | Description                                                   |
|---------------------------|----------------------|---------------------------------------------------------------|
| `DATABASE_URL`            | —                    | PostgreSQL connection string (required)                       |
| `REDIS_URL`               | `redis://localhost:6379` | Redis connection string                                   |
| `SERVER_PORT`             | `8000`               | Host port                                                     |
| `ADMIN_TOKEN`             | —                    | Bearer token for protected routes (**required in production**)|
| `RUST_LOG`                | `info`               | Log level — set to `debug` to bypass admin token check in dev |
| `INGEST_RATE_LIMIT`       | `120`                | Max events per API key per 60-second window                   |
| `EVENT_RETENTION_DAYS`    | `90`                 | Delete events older than N days (0 = disabled)                |
| `DATABASE_MAX_CONNECTIONS`| `30`                 | PostgreSQL connection pool size                               |
| `CORS_ORIGIN`             | `*` (any)            | Restrict CORS to a specific origin                            |
| `ANTHROPIC_API_KEY`       | *(empty)*            | Enable AI analysis via Claude                                 |
| `OPENAI_API_KEY`          | *(empty)*            | Enable AI analysis via GPT                                    |
| `GEMINI_API_KEY`          | *(empty)*            | Enable AI analysis via Gemini                                 |
| `SMTP_HOST`               | *(empty)*            | SMTP host for email alerts (leave blank to disable)           |
| `SMTP_PORT`               | `587`                | SMTP port                                                     |
| `SMTP_USER`               | —                    | SMTP username                                                 |
| `SMTP_PASS`               | —                    | SMTP password                                                 |
| `SMTP_FROM`               | `devpulse@localhost` | From address for alert emails                                 |

See [`.env.example`](.env.example) for a ready-to-copy template.

## Metrics

`GET /metrics` returns Prometheus-format counters:

```
devpulse_events_ingested_total
devpulse_issues_total
devpulse_projects_total
devpulse_jobs_processed_total
devpulse_jobs_failed_total
devpulse_dlq_depth            # dead-letter queue depth
```

## License

MIT — see [LICENSE](../LICENSE)
