use axum::{extract::{Path, State}, Json};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{errors::AppError, AppState};

// GET /api/projects
pub async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let projects = sqlx::query!(
        "SELECT id, name, api_key, platform, created_at FROM projects ORDER BY created_at DESC"
    )
    .fetch_all(&state.pg_pool)
    .await?;

    let data: Vec<Value> = projects.iter().map(|p| json!({
        "id":         p.id,
        "name":       p.name,
        "api_key":    p.api_key,
        "platform":   p.platform,
        "created_at": p.created_at,
    })).collect();

    Ok(Json(json!({ "data": data })))
}

// POST /api/projects
pub async fn create_project(
    State(state): State<AppState>,
    Json(body):   Json<Value>,
) -> Result<Json<Value>, AppError> {
    let name = body["name"]
        .as_str()
        .ok_or_else(|| AppError::BadRequest("name is required".into()))?;

    let platform = body["platform"].as_str().unwrap_or("php");

    // Generate a unique API key
    let api_key = Uuid::new_v4().simple().to_string();

    let project = sqlx::query!(
        "INSERT INTO projects (name, api_key, platform) VALUES ($1, $2, $3)
         RETURNING id, name, api_key, platform, created_at",
        name,
        api_key,
        platform
    )
    .fetch_one(&state.pg_pool)
    .await?;

    Ok(Json(json!({
        "id":         project.id,
        "name":       project.name,
        "api_key":    project.api_key,
        "platform":   project.platform,
        "created_at": project.created_at,
    })))
}

// POST /api/projects/:id/alerts
pub async fn create_alert(
    Path(project_id): Path<Uuid>,
    State(state):     State<AppState>,
    Json(body):       Json<Value>,
) -> Result<Json<Value>, AppError> {
    let channel  = body["channel"].as_str()
        .ok_or_else(|| AppError::BadRequest("channel is required".into()))?;
    let endpoint = body["endpoint"].as_str()
        .ok_or_else(|| AppError::BadRequest("endpoint is required".into()))?;

    if !["webhook", "telegram", "email"].contains(&channel) {
        return Err(AppError::BadRequest(
            "channel must be webhook, telegram or email".into()
        ));
    }

    let alert = sqlx::query!(
        "INSERT INTO alerts (project_id, channel, endpoint)
         VALUES ($1, $2, $3)
         RETURNING id, project_id, channel, endpoint, enabled",
        project_id,
        channel,
        endpoint
    )
    .fetch_one(&state.pg_pool)
    .await?;

    Ok(Json(json!({
        "id":         alert.id,
        "project_id": alert.project_id,
        "channel":    alert.channel,
        "endpoint":   alert.endpoint,
        "enabled":    alert.enabled,
    })))
}
