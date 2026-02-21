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
docker compose up -d
```

Open [http://localhost:8000](http://localhost:8000)

## Development

```bash
# Start infrastructure (Postgres, Redis) + Mailpit for email previews
docker compose -f docker-compose.dev.yaml up -d

# Run the server locally with hot-reload
cargo run

# Run the dashboard with HMR
cd dashboard && npm run dev
```

## Building

```bash
# Build production Docker image
docker compose build
```

The Dockerfile compiles the Rust binary and embeds the pre-built Vue dashboard into the binary via `include_dir!`.

## API Routes

| Method | Path                          | Auth   | Description                    |
|--------|-------------------------------|--------|--------------------------------|
| GET    | `/health`                     | None   | Health check                   |
| POST   | `/api/ingest/{api_key}`       | None   | Ingest an error event          |
| GET    | `/ws`                         | None   | WebSocket (live event stream)  |
| GET    | `/api/projects`               | Token  | List projects                  |
| POST   | `/api/projects`               | Token  | Create a project               |
| POST   | `/api/projects/{id}/alerts`   | Token  | Create an alert rule           |
| GET    | `/api/issues`                 | Token  | List issues                    |
| GET    | `/api/issues/{id}`            | Token  | Get a single issue             |
| PATCH  | `/api/issues/{id}`            | Token  | Update an issue                |
| DELETE | `/api/issues/{id}`            | Token  | Delete an issue                |
| GET    | `/api/stats`                  | Token  | Aggregated statistics          |

Protected routes require an `Authorization: Bearer <ADMIN_TOKEN>` header.

## Environment Variables

| Variable               | Default       | Description                                      |
|------------------------|---------------|--------------------------------------------------|
| `DATABASE_URL`         | —             | PostgreSQL connection string                     |
| `REDIS_URL`            | —             | Redis connection string                          |
| `SERVER_PORT`          | `8000`        | Host port                                        |
| `ADMIN_TOKEN`          | *(empty)*     | Bearer token for protected routes (leave blank in dev) |
| `RUST_LOG`             | `info`        | Log level (`error`, `warn`, `info`, `debug`, `trace`) |
| `INGEST_RATE_LIMIT`    | `120`         | Max events per API key per 60-second window      |
| `EVENT_RETENTION_DAYS` | `90`          | Delete events older than N days (0 = disabled)   |
| `SMTP_HOST`            | *(empty)*     | SMTP host for email alerts (leave blank to disable) |
| `SMTP_PORT`            | `587`         | SMTP port                                        |
| `SMTP_USER`            | —             | SMTP username                                    |
| `SMTP_PASS`            | —             | SMTP password                                    |
| `SMTP_FROM`            | `devpulse@localhost` | From address for alert emails            |

See [`.env.example`](.env.example) for the full list.

## License

MIT — see [LICENSE](../LICENSE)
