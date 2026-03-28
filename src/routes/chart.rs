use axum::{extract::State, Json};
use serde_json::{json, Value};
use sqlx::Row;
use crate::{errors::AppError, AppState};

/// GET /api/stats/chart — time-series and breakdown data for the analytics dashboard.
/// Uses runtime queries (not the query! macro) so no offline .sqlx/ files are needed.
pub async fn get_chart_stats(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    // ── Events ingested per day (last 14 days) ────────────────────────────────
    let event_rows = sqlx::query(
        "SELECT DATE_TRUNC('day', created_at)::DATE AS day, COUNT(*)::BIGINT AS count \
         FROM events \
         WHERE created_at > NOW() - INTERVAL '14 days' \
         GROUP BY 1 ORDER BY 1",
    )
    .fetch_all(&state.pg_pool)
    .await?;

    // ── New issues opened per day (last 14 days) ──────────────────────────────
    let issue_rows = sqlx::query(
        "SELECT DATE_TRUNC('day', first_seen)::DATE AS day, COUNT(*)::BIGINT AS count \
         FROM issues \
         WHERE first_seen > NOW() - INTERVAL '14 days' \
         GROUP BY 1 ORDER BY 1",
    )
    .fetch_all(&state.pg_pool)
    .await?;

    // ── Unresolved issues broken down by level ────────────────────────────────
    let level_rows = sqlx::query(
        "SELECT level, COUNT(*)::BIGINT AS count \
         FROM issues WHERE status = 'unresolved' GROUP BY level",
    )
    .fetch_all(&state.pg_pool)
    .await?;

    // ── Top 6 projects by event volume in the last 7 days ────────────────────
    let project_rows = sqlx::query(
        "SELECT p.name, COUNT(e.id)::BIGINT AS count \
         FROM events e \
         JOIN issues i ON e.issue_id = i.id \
         JOIN projects p ON i.project_id = p.id \
         WHERE e.created_at > NOW() - INTERVAL '7 days' \
         GROUP BY p.name ORDER BY count DESC LIMIT 6",
    )
    .fetch_all(&state.pg_pool)
    .await?;

    // ── Issues by status (for the status doughnut) ────────────────────────────
    let status_rows = sqlx::query(
        "SELECT status, COUNT(*)::BIGINT AS count FROM issues GROUP BY status",
    )
    .fetch_all(&state.pg_pool)
    .await?;

    Ok(Json(json!({
        "events_by_day": event_rows.iter().map(|r| json!({
            "day":   r.get::<chrono::NaiveDate, _>("day").to_string(),
            "count": r.get::<i64, _>("count"),
        })).collect::<Vec<_>>(),

        "issues_by_day": issue_rows.iter().map(|r| json!({
            "day":   r.get::<chrono::NaiveDate, _>("day").to_string(),
            "count": r.get::<i64, _>("count"),
        })).collect::<Vec<_>>(),

        "by_level": level_rows.iter().map(|r| json!({
            "level": r.get::<String, _>("level"),
            "count": r.get::<i64, _>("count"),
        })).collect::<Vec<_>>(),

        "by_status": status_rows.iter().map(|r| json!({
            "status": r.get::<String, _>("status"),
            "count":  r.get::<i64, _>("count"),
        })).collect::<Vec<_>>(),

        "top_projects": project_rows.iter().map(|r| json!({
            "name":  r.get::<String, _>("name"),
            "count": r.get::<i64, _>("count"),
        })).collect::<Vec<_>>(),
    })))
}
