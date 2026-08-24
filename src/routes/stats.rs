use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{errors::AppError, AppState};

const STATS_CACHE_PREFIX: &str = "devpulse:stats_cache";
const STATS_CACHE_TTL: u64  = 30; // seconds

#[derive(Deserialize)]
pub struct StatsParams {
    pub project_id: Option<Uuid>,
}

/// GET /api/stats?project_id= — aggregate counts for the dashboard header.
/// Omitting project_id returns global counts across every project. Results
/// are cached in Redis for 30 seconds (one cache entry per project_id, plus
/// one for the global view) to avoid hammering Postgres.
pub async fn get_stats(
    State(state): State<AppState>,
    Query(params): Query<StatsParams>,
) -> Result<Json<Value>, AppError> {
    let cache_key = match params.project_id {
        Some(id) => format!("{STATS_CACHE_PREFIX}:{id}"),
        None      => format!("{STATS_CACHE_PREFIX}:global"),
    };

    // Try cache first
    if let Ok(mut conn) = state.redis_pool.get().await {
        if let Ok(cached) = deadpool_redis::redis::cmd("GET")
            .arg(&cache_key)
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
        WHERE ($1::uuid IS NULL OR project_id = $1)
        "#,
        params.project_id as Option<Uuid>,
    )
    .fetch_one(&state.pg_pool)
    .await?;

    let events = sqlx::query!(
        r#"
        SELECT COUNT(*) AS total FROM events
        WHERE created_at > NOW() - INTERVAL '24 hours'
          AND ($1::uuid IS NULL OR project_id = $1)
        "#,
        params.project_id as Option<Uuid>,
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
                .arg(&cache_key)
                .arg(STATS_CACHE_TTL)
                .arg(serialized)
                .query_async(&mut *conn)
                .await;
        }
    }

    Ok(Json(result))
}
