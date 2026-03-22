use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::{errors::AppError, AppState};

const STATS_CACHE_KEY: &str = "devpulse:stats_cache";
const STATS_CACHE_TTL: u64  = 30; // seconds

/// GET /api/stats — aggregate counts for the dashboard header.
/// Results are cached in Redis for 30 seconds to avoid hammering Postgres.
pub async fn get_stats(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    // Try cache first
    if let Ok(mut conn) = state.redis_pool.get().await {
        if let Ok(cached) = deadpool_redis::redis::cmd("GET")
            .arg(STATS_CACHE_KEY)
            .query_async::<String>(&mut *conn)
            .await
        {
            if let Ok(v) = serde_json::from_str::<Value>(&cached) {
                return Ok(Json(v));
            }
        }
    }

    // Cache miss — query Postgres
    let issues = sqlx::query!(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE status = 'unresolved')                          AS unresolved,
            COUNT(*) FILTER (WHERE status = 'resolved')                            AS resolved,
            COUNT(*) FILTER (WHERE status = 'ignored')                             AS ignored,
            COUNT(*) FILTER (WHERE last_seen  > NOW() - INTERVAL '24 hours'
                               AND status = 'unresolved')                          AS new_24h,
            COUNT(*) FILTER (WHERE first_seen < NOW() - INTERVAL '24 hours'
                               AND last_seen  > NOW() - INTERVAL '24 hours'
                               AND status = 'unresolved')                          AS regressions_24h
        FROM issues
        "#
    )
    .fetch_one(&state.pg_pool)
    .await?;

    let events = sqlx::query!(
        "SELECT COUNT(*) AS total FROM events WHERE created_at > NOW() - INTERVAL '24 hours'"
    )
    .fetch_one(&state.pg_pool)
    .await?;

    let result = json!({
        "issues": {
            "unresolved":      issues.unresolved.unwrap_or(0),
            "resolved":        issues.resolved.unwrap_or(0),
            "ignored":         issues.ignored.unwrap_or(0),
            "new_24h":         issues.new_24h.unwrap_or(0),
            "regressions_24h": issues.regressions_24h.unwrap_or(0),
        },
        "events_24h": events.total.unwrap_or(0),
    });

    // Write to cache
    if let Ok(mut conn) = state.redis_pool.get().await {
        if let Ok(serialized) = serde_json::to_string(&result) {
            let _: Result<(), _> = deadpool_redis::redis::cmd("SETEX")
                .arg(STATS_CACHE_KEY)
                .arg(STATS_CACHE_TTL)
                .arg(serialized)
                .query_async(&mut *conn)
                .await;
        }
    }

    Ok(Json(result))
}
