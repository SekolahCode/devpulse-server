/// GET /metrics — Prometheus text-format endpoint.
/// Exposes key operational counters without requiring an external crate.
use axum::{extract::State, http::header, response::Response, body::Body};
use crate::AppState;

pub async fn metrics(State(state): State<AppState>) -> Response<Body> {
    let mut lines: Vec<String> = Vec::new();

    // ── Issue counts ─────────────────────────────────────────────────────────
    if let Ok(row) = sqlx::query!(
        r#"SELECT
            COUNT(*) FILTER (WHERE status = 'unresolved') AS unresolved,
            COUNT(*) FILTER (WHERE status = 'resolved')   AS resolved,
            COUNT(*) FILTER (WHERE status = 'ignored')    AS ignored
           FROM issues"#
    )
    .fetch_one(&state.pg_pool)
    .await
    {
        lines.push("# HELP devpulse_issues_total Total issues by status".into());
        lines.push("# TYPE devpulse_issues_total gauge".into());
        lines.push(format!("devpulse_issues_total{{status=\"unresolved\"}} {}", row.unresolved.unwrap_or(0)));
        lines.push(format!("devpulse_issues_total{{status=\"resolved\"}} {}", row.resolved.unwrap_or(0)));
        lines.push(format!("devpulse_issues_total{{status=\"ignored\"}} {}", row.ignored.unwrap_or(0)));
    }

    // ── Events last 24 h ─────────────────────────────────────────────────────
    if let Ok(row) = sqlx::query!(
        "SELECT COUNT(*) AS total FROM events WHERE created_at > NOW() - INTERVAL '24 hours'"
    )
    .fetch_one(&state.pg_pool)
    .await
    {
        lines.push("# HELP devpulse_events_24h Events ingested in the last 24 hours".into());
        lines.push("# TYPE devpulse_events_24h gauge".into());
        lines.push(format!("devpulse_events_24h {}", row.total.unwrap_or(0)));
    }

    // ── Project count ────────────────────────────────────────────────────────
    if let Ok(row) = sqlx::query!("SELECT COUNT(*) AS total FROM projects")
        .fetch_one(&state.pg_pool)
        .await
    {
        lines.push("# HELP devpulse_projects_total Total configured projects".into());
        lines.push("# TYPE devpulse_projects_total gauge".into());
        lines.push(format!("devpulse_projects_total {}", row.total.unwrap_or(0)));
    }

    // ── Redis queue depth ────────────────────────────────────────────────────
    if let Ok(mut conn) = state.redis_pool.get().await {
        if let Ok(depth) = deadpool_redis::redis::cmd("LLEN")
            .arg("devpulse:events")
            .query_async::<i64>(&mut *conn)
            .await
        {
            lines.push("# HELP devpulse_queue_depth Current event queue depth".into());
            lines.push("# TYPE devpulse_queue_depth gauge".into());
            lines.push(format!("devpulse_queue_depth {}", depth));
        }
    }

    // ── DB pool stats ────────────────────────────────────────────────────────
    let pool_size = state.pg_pool.size();
    let pool_idle = state.pg_pool.num_idle();
    lines.push("# HELP devpulse_db_pool_size Total DB connections in pool".into());
    lines.push("# TYPE devpulse_db_pool_size gauge".into());
    lines.push(format!("devpulse_db_pool_size {}", pool_size));
    lines.push("# HELP devpulse_db_pool_idle Idle DB connections".into());
    lines.push("# TYPE devpulse_db_pool_idle gauge".into());
    lines.push(format!("devpulse_db_pool_idle {}", pool_idle));

    // ── Pending alert retries ────────────────────────────────────────────────
    if let Ok(row) = sqlx::query!(
        "SELECT COUNT(*) AS total FROM alerts WHERE retry_after IS NOT NULL AND retry_after > NOW()"
    )
    .fetch_one(&state.pg_pool)
    .await
    {
        lines.push("# HELP devpulse_alerts_pending_retry Alerts waiting for retry".into());
        lines.push("# TYPE devpulse_alerts_pending_retry gauge".into());
        lines.push(format!("devpulse_alerts_pending_retry {}", row.total.unwrap_or(0)));
    }

    lines.push(String::new()); // trailing newline
    let body = lines.join("\n");

    Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")
        .body(Body::from(body))
        .unwrap()
}
