use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use crate::AppState;

/// GET /health — checks live connectivity to Postgres and Redis.
/// Returns 200 when all dependencies are healthy, 503 otherwise.
/// Used by Docker HEALTHCHECK and load-balancer probes.
pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    let pg_ok = sqlx::query("SELECT 1")
        .execute(&state.pg_pool)
        .await
        .is_ok();

    let redis_ok = state.redis_pool.get().await.is_ok();

    let body = json!({
        "status":   if pg_ok && redis_ok { "ok" } else { "degraded" },
        "postgres": if pg_ok { "ok" } else { "error" },
        "redis":    if redis_ok { "ok" } else { "error" },
    });

    let status = if pg_ok && redis_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status, Json(body))
}
