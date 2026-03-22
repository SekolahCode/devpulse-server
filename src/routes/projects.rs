use axum::{extract::{Path, State}, Json};
use rand::{rngs::OsRng, RngCore};
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

    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    let api_key = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

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

// PATCH /api/projects/:id  — rename or change platform
pub async fn update_project(
    Path(id):     Path<Uuid>,
    State(state): State<AppState>,
    Json(body):   Json<Value>,
) -> Result<Json<Value>, AppError> {
    let name = body["name"]
        .as_str()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| AppError::BadRequest("name is required".into()))?;

    let platform = body["platform"].as_str();

    let row = sqlx::query!(
        "UPDATE projects SET name = $1, platform = COALESCE($2, platform) WHERE id = $3
         RETURNING id, name, api_key, platform, created_at",
        name,
        platform as Option<&str>,
        id
    )
    .fetch_optional(&state.pg_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Project not found".into()))?;

    Ok(Json(json!({
        "id":         row.id,
        "name":       row.name,
        "api_key":    row.api_key,
        "platform":   row.platform,
        "created_at": row.created_at,
    })))
}

// POST /api/projects/:id/rotate-key — generate a new API key
pub async fn rotate_api_key(
    Path(id):     Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    let new_key = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    let row = sqlx::query!(
        "UPDATE projects SET api_key = $1 WHERE id = $2 RETURNING id, name, api_key, platform",
        new_key, id
    )
    .fetch_optional(&state.pg_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Project not found".into()))?;

    tracing::info!("API key rotated for project {}", id);
    Ok(Json(json!({ "id": row.id, "name": row.name, "api_key": row.api_key, "platform": row.platform })))
}

// GET /api/projects/:id/releases
pub async fn list_releases(
    Path(project_id): Path<Uuid>,
    State(state):     State<AppState>,
) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT
            r.id, r.version, r.ref, r.url, r.deployed_at,
            COUNT(i.id) FILTER (WHERE i.first_release = r.version)                                    AS new_issues,
            COUNT(i.id) FILTER (WHERE i.last_release  = r.version AND i.status = 'unresolved')        AS open_issues,
            COUNT(i.id) FILTER (WHERE i.last_release  = r.version AND i.status = 'resolved')          AS resolved_issues
        FROM releases r
        LEFT JOIN issues i ON i.project_id = r.project_id
        WHERE r.project_id = $1
        GROUP BY r.id, r.version, r.ref, r.url, r.deployed_at
        ORDER BY r.deployed_at DESC
        LIMIT 100
        "#,
        project_id
    )
    .fetch_all(&state.pg_pool)
    .await?;

    let data: Vec<Value> = rows.iter().map(|r| json!({
        "id":               r.id,
        "version":          r.version,
        "ref":              r.r#ref,
        "url":              r.url,
        "deployed_at":      r.deployed_at,
        "new_issues":       r.new_issues.unwrap_or(0),
        "open_issues":      r.open_issues.unwrap_or(0),
        "resolved_issues":  r.resolved_issues.unwrap_or(0),
    })).collect();

    Ok(Json(json!({ "data": data })))
}

// POST /api/projects/:id/releases
pub async fn create_release(
    Path(project_id): Path<Uuid>,
    State(state):     State<AppState>,
    Json(body):       Json<Value>,
) -> Result<Json<Value>, AppError> {
    let version = body["version"].as_str()
        .ok_or_else(|| AppError::BadRequest("version is required".into()))?;

    let row = sqlx::query!(
        r#"INSERT INTO releases (project_id, version, ref, url)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (project_id, version) DO UPDATE SET
               ref = COALESCE(EXCLUDED.ref, releases.ref),
               url = COALESCE(EXCLUDED.url, releases.url),
               deployed_at = NOW()
           RETURNING id, version, ref, url, deployed_at"#,
        project_id,
        version,
        body["ref"].as_str() as Option<&str>,
        body["url"].as_str() as Option<&str>,
    )
    .fetch_one(&state.pg_pool)
    .await?;

    Ok(Json(json!({
        "id":          row.id,
        "version":     row.version,
        "ref":         row.r#ref,
        "url":         row.url,
        "deployed_at": row.deployed_at,
    })))
}

// DELETE /api/projects/:id
pub async fn delete_project(
    Path(id):     Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query!("DELETE FROM projects WHERE id = $1", id)
        .execute(&state.pg_pool)
        .await?
        .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound("Project not found".into()));
    }

    Ok(Json(json!({ "deleted": true, "id": id })))
}

// GET /api/projects/:id/alerts
pub async fn list_alerts(
    Path(project_id): Path<Uuid>,
    State(state):     State<AppState>,
) -> Result<Json<Value>, AppError> {
    let alerts = sqlx::query!(
        r#"
        SELECT id, channel, endpoint, enabled, cooldown_minutes, last_alerted_at
        FROM alerts
        WHERE project_id = $1
        ORDER BY created_at DESC
        "#,
        project_id
    )
    .fetch_all(&state.pg_pool)
    .await?;

    let data: Vec<Value> = alerts.iter().map(|a| json!({
        "id":               a.id,
        "channel":          a.channel,
        "endpoint":         a.endpoint,
        "enabled":          a.enabled,
        "cooldown_minutes": a.cooldown_minutes,
        "last_alerted_at":  a.last_alerted_at,
    })).collect();

    Ok(Json(json!({ "data": data })))
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

    // Verify project exists
    let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM projects WHERE id = $1)", project_id)
        .fetch_one(&state.pg_pool)
        .await?
        .unwrap_or(false);

    if !exists {
        return Err(AppError::NotFound("Project not found".into()));
    }

    let alert = sqlx::query!(
        "INSERT INTO alerts (project_id, channel, endpoint)
         VALUES ($1, $2, $3)
         RETURNING id, project_id, channel, endpoint, enabled, cooldown_minutes",
        project_id,
        channel,
        endpoint
    )
    .fetch_one(&state.pg_pool)
    .await?;

    Ok(Json(json!({
        "id":               alert.id,
        "project_id":       alert.project_id,
        "channel":          alert.channel,
        "endpoint":         alert.endpoint,
        "enabled":          alert.enabled,
        "cooldown_minutes": alert.cooldown_minutes,
    })))
}

// PATCH /api/alerts/:id  — toggle enabled or change cooldown
pub async fn update_alert(
    Path(alert_id): Path<Uuid>,
    State(state):   State<AppState>,
    Json(body):     Json<Value>,
) -> Result<Json<Value>, AppError> {
    // At least one field required
    if body["enabled"].is_null() && body["cooldown_minutes"].is_null() && body["endpoint"].is_null() {
        return Err(AppError::BadRequest("provide enabled, cooldown_minutes, or endpoint".into()));
    }

    let row = sqlx::query!(
        r#"
        UPDATE alerts SET
            enabled          = COALESCE($1, enabled),
            cooldown_minutes = COALESCE($2, cooldown_minutes),
            endpoint         = COALESCE($3, endpoint)
        WHERE id = $4
        RETURNING id, channel, endpoint, enabled, cooldown_minutes
        "#,
        body["enabled"].as_bool(),
        body["cooldown_minutes"].as_i64().map(|v| v as i32),
        body["endpoint"].as_str(),
        alert_id
    )
    .fetch_optional(&state.pg_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Alert not found".into()))?;

    Ok(Json(json!({
        "id":               row.id,
        "channel":          row.channel,
        "endpoint":         row.endpoint,
        "enabled":          row.enabled,
        "cooldown_minutes": row.cooldown_minutes,
    })))
}

// DELETE /api/alerts/:id
pub async fn delete_alert(
    Path(alert_id): Path<Uuid>,
    State(state):   State<AppState>,
) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query!("DELETE FROM alerts WHERE id = $1", alert_id)
        .execute(&state.pg_pool)
        .await?
        .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound("Alert not found".into()));
    }

    Ok(Json(json!({ "deleted": true, "id": alert_id })))
}

// DELETE /api/releases/:id
pub async fn delete_release(
    Path(release_id): Path<Uuid>,
    State(state):     State<AppState>,
) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query!("DELETE FROM releases WHERE id = $1", release_id)
        .execute(&state.pg_pool)
        .await?
        .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound("Release not found".into()));
    }

    Ok(Json(json!({ "deleted": true, "id": release_id })))
}
