use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use crate::{
    errors::AppError,
    models::IngestPayload,
    queue::{push_job, EventJob},
    AppState,
};

pub async fn handle_ingest(
    headers:        HeaderMap,
    State(state):   State<AppState>,
    Json(payload):  Json<IngestPayload>,
) -> Result<StatusCode, AppError> {

    // 1. Extract API key from X-API-Key header
    let api_key = headers
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::Unauthorized("Missing X-API-Key header".into()))?;

    // 2. Rate limit per API key (Redis-backed, configured via INGEST_RATE_LIMIT)
    if !state.rate_limiter.check(&api_key).await {
        return Err(AppError::TooManyRequests);
    }

    // 3. Validate API key
    let project = sqlx::query!(
        "SELECT id FROM projects WHERE api_key = $1",
        api_key
    )
    .fetch_optional(&state.pg_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Invalid API key".to_string()))?;

    // 4. Basic payload validation — require at least one of exception or message
    if payload.exception.is_none() && payload.message.is_none() {
        return Err(AppError::BadRequest(
            "payload must include either 'exception' or 'message'".into(),
        ));
    }

    // 5. Push to Redis queue — return immediately (fire-and-forget)
    push_job(&state.redis_pool, EventJob {
        project_id: project.id,
        payload,
    })
    .await
    .map_err(|e| AppError::BadRequest(e))?;

    tracing::info!("📥 Event queued for project: {}", project.id);

    Ok(StatusCode::ACCEPTED)
}
