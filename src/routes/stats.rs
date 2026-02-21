use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::{errors::AppError, AppState};

/// GET /api/stats — aggregate counts for the dashboard header.
pub async fn get_stats(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
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

    Ok(Json(json!({
        "issues": {
            "unresolved":      issues.unresolved.unwrap_or(0),
            "resolved":        issues.resolved.unwrap_or(0),
            "ignored":         issues.ignored.unwrap_or(0),
            "new_24h":         issues.new_24h.unwrap_or(0),
            "regressions_24h": issues.regressions_24h.unwrap_or(0),
        },
        "events_24h": events.total.unwrap_or(0),
    })))
}
