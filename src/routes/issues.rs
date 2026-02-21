use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{errors::AppError, AppState};

#[derive(Deserialize)]
pub struct IssueQuery {
    pub project_id: Option<Uuid>,
    pub status:     Option<String>,   // unresolved | resolved | ignored
    pub level:      Option<String>,   // error | warning | info
    pub search:     Option<String>,   // full-text search on title
    pub limit:      Option<i64>,
    pub offset:     Option<i64>,
}

// GET /api/issues?project_id=&status=&level=&search=&limit=&offset=
pub async fn list_issues(
    State(state): State<AppState>,
    Query(params): Query<IssueQuery>,
) -> Result<Json<Value>, AppError> {
    let limit  = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0).max(0);
    let status = params.status.unwrap_or_else(|| "unresolved".into());

    // Normalise search: None → NULL in Postgres
    let search = params.search.as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| format!("%{}%", s));

    let issues = sqlx::query!(
        r#"
        SELECT
            i.id, i.project_id, i.fingerprint, i.title,
            i.level, i.status, i.first_seen, i.last_seen, i.event_count,
            p.name AS project_name
        FROM issues i
        JOIN projects p ON p.id = i.project_id
        WHERE ($1::uuid IS NULL OR i.project_id = $1)
          AND i.status = $2
          AND ($3::text IS NULL OR i.level  = $3)
          AND ($4::text IS NULL OR i.title ILIKE $4)
        ORDER BY i.last_seen DESC
        LIMIT $5 OFFSET $6
        "#,
        params.project_id as Option<Uuid>,
        status,
        params.level.clone() as Option<String>,
        search.clone() as Option<String>,
        limit,
        offset
    )
    .fetch_all(&state.pg_pool)
    .await?;

    // Total count for the same filters (so the frontend knows when to stop)
    let total = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM issues i
        WHERE ($1::uuid IS NULL OR i.project_id = $1)
          AND i.status = $2
          AND ($3::text IS NULL OR i.level  = $3)
          AND ($4::text IS NULL OR i.title ILIKE $4)
        "#,
        params.project_id as Option<Uuid>,
        status,
        params.level as Option<String>,
        search as Option<String>
    )
    .fetch_one(&state.pg_pool)
    .await?
    .unwrap_or(0);

    let data: Vec<Value> = issues.iter().map(|i| json!({
        "id":           i.id,
        "project_id":   i.project_id,
        "project_name": i.project_name,
        "title":        i.title,
        "level":        i.level,
        "status":       i.status,
        "first_seen":   i.first_seen,
        "last_seen":    i.last_seen,
        "event_count":  i.event_count,
    })).collect();

    Ok(Json(json!({
        "data":   data,
        "total":  total,
        "limit":  limit,
        "offset": offset,
    })))
}

// GET /api/issues/:id
pub async fn get_issue(
    Path(id):     Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let issue = sqlx::query!(
        r#"
        SELECT
            i.id, i.project_id, i.fingerprint, i.title,
            i.level, i.status, i.first_seen, i.last_seen, i.event_count,
            p.name AS project_name, p.platform
        FROM issues i
        JOIN projects p ON p.id = i.project_id
        WHERE i.id = $1
        "#,
        id
    )
    .fetch_optional(&state.pg_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Issue not found".into()))?;

    // Latest 10 events (with environment)
    let events = sqlx::query!(
        r#"
        SELECT id, payload, context, environment, created_at
        FROM events
        WHERE issue_id = $1
        ORDER BY created_at DESC
        LIMIT 10
        "#,
        id
    )
    .fetch_all(&state.pg_pool)
    .await?;

    let events_data: Vec<Value> = events.iter().map(|e| json!({
        "id":          e.id,
        "payload":     e.payload,
        "context":     e.context,
        "environment": e.environment,
        "created_at":  e.created_at,
    })).collect();

    Ok(Json(json!({
        "id":           issue.id,
        "project_id":   issue.project_id,
        "project_name": issue.project_name,
        "platform":     issue.platform,
        "fingerprint":  issue.fingerprint,
        "title":        issue.title,
        "level":        issue.level,
        "status":       issue.status,
        "first_seen":   issue.first_seen,
        "last_seen":    issue.last_seen,
        "event_count":  issue.event_count,
        "events":       events_data,
    })))
}

// PATCH /api/issues/:id  —  { "status": "resolved" | "ignored" | "unresolved" }
pub async fn update_issue(
    Path(id):     Path<Uuid>,
    State(state): State<AppState>,
    Json(body):   Json<Value>,
) -> Result<Json<Value>, AppError> {
    let status = body["status"]
        .as_str()
        .ok_or_else(|| AppError::BadRequest("status is required".into()))?;

    if !["resolved", "ignored", "unresolved"].contains(&status) {
        return Err(AppError::BadRequest(
            "status must be resolved, ignored, or unresolved".into()
        ));
    }

    sqlx::query!(
        "UPDATE issues SET status = $1 WHERE id = $2",
        status, id
    )
    .execute(&state.pg_pool)
    .await?;

    Ok(Json(json!({ "id": id, "status": status })))
}

// DELETE /api/issues/:id
pub async fn delete_issue(
    Path(id):     Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    sqlx::query!("DELETE FROM issues WHERE id = $1", id)
        .execute(&state.pg_pool)
        .await?;

    Ok(Json(json!({ "deleted": true, "id": id })))
}
