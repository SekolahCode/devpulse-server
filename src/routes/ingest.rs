use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::{
    errors::AppError,
    models::IngestPayload,
    queue::{push_job, EventJob},
    AppState,
};

pub async fn handle_ingest(
    Path(api_key): Path<String>,
    State(state):  State<AppState>,
    Json(payload): Json<IngestPayload>,
) -> Result<StatusCode, AppError> {

    // 1. Rate limit per API key (sliding window, configured via INGEST_RATE_LIMIT)
    if !state.rate_limiter.check(&api_key) {
        return Err(AppError::TooManyRequests);
    }

    // 2. Validate API key
    let project = sqlx::query!(
        "SELECT id FROM projects WHERE api_key = $1",
        api_key
    )
    .fetch_optional(&state.pg_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Invalid API key".to_string()))?;

    // 3. Push to Redis queue — return immediately (fire-and-forget)
    push_job(&state.redis_pool, EventJob {
        project_id: project.id,
        payload,
    })
    .await
    .map_err(|e| AppError::BadRequest(e))?;

    tracing::info!("📥 Event queued for project: {}", project.id);

    Ok(StatusCode::ACCEPTED)
}
