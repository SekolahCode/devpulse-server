mod ai;
mod alerts;
mod auth;
mod db;
mod errors;
mod models;
mod queue;
mod rate_limit;
mod retention;
mod routes;
mod trace_id;
mod worker;
mod ws;

use axum::{
    body::Body,
    extract::{DefaultBodyLimit, OriginalUri},
    http::{header, Method, StatusCode},
    middleware,
    response::Response,
    routing::{delete, get, patch, post},
    Router,
};
use tower_http::cors::CorsLayer;
use deadpool_redis::{Config as RedisConfig, Runtime};
use include_dir::{include_dir, Dir};
use rate_limit::RateLimiter;
use reqwest::Client;
use std::sync::{atomic::AtomicU64, Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static WEB_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/web/dist");

async fn serve_spa(OriginalUri(uri): OriginalUri) -> Response<Body> {
    let path = uri.path().trim_start_matches('/');

    if let Some(file) = WEB_DIR.get_file(path) {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime_for(path))
            .body(Body::from(file.contents()))
            .unwrap()
    } else {
        let index = WEB_DIR.get_file("index.html").unwrap();
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(Body::from(index.contents()))
            .unwrap()
    }
}

fn mime_for(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html"        => "text/html; charset=utf-8",
        "js" | "mjs"  => "application/javascript",
        "css"         => "text/css",
        "svg"         => "image/svg+xml",
        "png"         => "image/png",
        "jpg" | "jpeg"=> "image/jpeg",
        "gif"         => "image/gif",
        "ico"         => "image/x-icon",
        "woff"        => "font/woff",
        "woff2"       => "font/woff2",
        "ttf"         => "font/ttf",
        "json"        => "application/json",
        "txt"         => "text/plain",
        _             => "application/octet-stream",
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pg_pool:         sqlx::PgPool,
    pub redis_pool:      deadpool_redis::Pool,
    pub redis_url:       String,             // for WS pub/sub dedicated connections
    pub rate_limiter:    Arc<RateLimiter>,
    pub http_client:     Client,
    pub anthropic_key:   Option<String>,
    pub openai_key:      Option<String>,
    pub gemini_key:      Option<String>,
    /// Cumulative count of successfully processed worker jobs since startup.
    pub jobs_processed:  Arc<AtomicU64>,
    /// Cumulative count of worker jobs that failed and were dead-lettered.
    pub jobs_failed:     Arc<AtomicU64>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── PostgreSQL ────────────────────────────────────────────────────────────
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_pool = db::create_pool(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .expect("Failed to run database migrations");
    tracing::info!("✅ Database migrations applied");

    // ── Redis ─────────────────────────────────────────────────────────────────
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".into());
    let redis_pool = RedisConfig::from_url(&redis_url)
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    // ── Rate limiter (Redis-backed, per-API-key) ──────────────────────────────
    let rate_limit = std::env::var("INGEST_RATE_LIMIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(120usize);
    let rate_limiter = Arc::new(RateLimiter::new(redis_pool.clone(), rate_limit, 60));

    // ── Shared HTTP client (for AI analysis + alerts) ─────────────────────────
    let http_client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");

    let anthropic_key = std::env::var("ANTHROPIC_API_KEY").ok().filter(|k| !k.is_empty());
    let openai_key    = std::env::var("OPENAI_API_KEY").ok().filter(|k| !k.is_empty());
    let gemini_key    = std::env::var("GEMINI_API_KEY").ok().filter(|k| !k.is_empty());

    let ai_providers: Vec<&str> = [
        anthropic_key.as_ref().map(|_| "Anthropic"),
        openai_key.as_ref().map(|_| "OpenAI"),
        gemini_key.as_ref().map(|_| "Google"),
    ].into_iter().flatten().collect();

    if ai_providers.is_empty() {
        tracing::warn!("⚠️  No AI provider keys set — AI analysis endpoint will return 400");
    } else {
        tracing::info!("🤖 AI analysis enabled ({})", ai_providers.join(", "));
    }

    let jobs_processed = Arc::new(AtomicU64::new(0));
    let jobs_failed    = Arc::new(AtomicU64::new(0));

    let state = AppState {
        pg_pool:       pg_pool.clone(),
        redis_pool:    redis_pool.clone(),
        redis_url:     redis_url.clone(),
        rate_limiter,
        http_client,
        anthropic_key,
        openai_key,
        gemini_key,
        jobs_processed: jobs_processed.clone(),
        jobs_failed:    jobs_failed.clone(),
    };

    // ── Background: event worker ──────────────────────────────────────────────
    tokio::spawn(worker::run(redis_pool.clone(), pg_pool.clone(), jobs_processed, jobs_failed));
    tracing::info!("⚙️  Event worker spawned");

    // ── Background: event retention ───────────────────────────────────────────
    let retention_days = std::env::var("EVENT_RETENTION_DAYS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(90i64);
    let pool_ret = pg_pool.clone();
    tokio::spawn(async move {
        retention::prune(&pool_ret, retention_days).await;
        retention::run(pool_ret, retention_days).await;
    });
    tracing::info!("🗑️  Retention job spawned (keep {} days)", retention_days);

    // ── Router ────────────────────────────────────────────────────────────────
    let protected = Router::new()
        // Projects
        .route("/api/projects",                    get(routes::projects::list_projects))
        .route("/api/projects",                    post(routes::projects::create_project))
        .route("/api/projects/{id}",               patch(routes::projects::update_project))
        .route("/api/projects/{id}",               delete(routes::projects::delete_project))
        .route("/api/projects/{id}/rotate-key",    post(routes::projects::rotate_api_key))
        // Releases
        .route("/api/projects/{id}/releases",      get(routes::projects::list_releases))
        .route("/api/projects/{id}/releases",      post(routes::projects::create_release))
        // Releases (delete)
        .route("/api/releases/{id}",               delete(routes::projects::delete_release))
        // Alerts
        .route("/api/projects/{id}/alerts",        get(routes::projects::list_alerts))
        .route("/api/projects/{id}/alerts",        post(routes::projects::create_alert))
        .route("/api/alerts/{id}",                 patch(routes::projects::update_alert))
        .route("/api/alerts/{id}",                 delete(routes::projects::delete_alert))
        // Issues
        .route("/api/issues",                      get(routes::issues::list_issues))
        .route("/api/issues/{id}",                 get(routes::issues::get_issue))
        .route("/api/issues/{id}",                 patch(routes::issues::update_issue))
        .route("/api/issues/{id}",                 delete(routes::issues::delete_issue))
        // AI analysis
        .route("/api/issues/{id}/analyze",         get(routes::analysis::get_analysis))
        .route("/api/issues/{id}/analyze",         post(routes::analysis::analyze_issue))
        .route("/api/ai/providers",                get(routes::providers::get_providers))
        // Stats
        .route("/api/stats",                       get(routes::stats::get_stats))
        .route("/api/stats/chart",                 get(routes::chart::get_chart_stats))
        .layer(middleware::from_fn(auth::require_admin_token));

    let cors = {
        // X-API-Key must be in allow_headers so browser pre-flight (OPTIONS) passes.
        // X-Trace-Id must be in allow_headers (inbound) and expose_headers (outbound)
        // so callers can both propagate and read back their trace ID.
        let x_api_key   = header::HeaderName::from_static("x-api-key");
        let x_trace_id  = header::HeaderName::from_static("x-trace-id");
        let layer = CorsLayer::new()
            .allow_methods([Method::POST, Method::GET, Method::OPTIONS])
            .allow_headers([header::CONTENT_TYPE, x_api_key, x_trace_id.clone()])
            .expose_headers([x_trace_id]);
        if let Ok(origin) = std::env::var("CORS_ORIGIN") {
            match origin.parse::<axum::http::HeaderValue>() {
                Ok(v)  => layer.allow_origin(v),
                Err(_) => {
                    tracing::warn!("CORS_ORIGIN '{}' is not valid — using wildcard", origin);
                    layer.allow_origin(tower_http::cors::Any)
                }
            }
        } else {
            layer.allow_origin(tower_http::cors::Any)
        }
    };

    let app = Router::new()
        .route("/health",               get(routes::health::health_check))
        .route("/metrics",              get(routes::metrics::metrics))
        .route("/api/ingest", post(routes::ingest::handle_ingest)
            // Ingest body capped at 256 KB — prevents OOM from malicious payloads
            .layer(DefaultBodyLimit::max(256 * 1024)))
        .route("/ws",                   get(ws::ws_handler))
        .merge(protected)
        .fallback(serve_spa)
        .layer(cors)
        // Attach a UUID trace ID to every request (sourced from X-Trace-Id header
        // when present, otherwise generated). Echoed back as X-Trace-Id response header.
        .layer(middleware::from_fn(trace_id::trace_id_middleware))
        // Global body limit for all other routes: 64 KB
        .layer(DefaultBodyLimit::max(64 * 1024))
        .with_state(state);

    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8000".into());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("🚀 DevPulse listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c    => {},
        _ = terminate => {},
    }
    tracing::info!("Shutdown signal received — draining in-flight requests…");
}
