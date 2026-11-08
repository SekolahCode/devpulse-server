use axum::{
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use std::net::SocketAddr;
use crate::{
    errors::AppError,
    models::IngestPayload,
    queue::{push_job, EventJob},
    AppState,
};

pub async fn handle_ingest(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers:        HeaderMap,
    State(state):   State<AppState>,
    Json(payload):  Json<IngestPayload>,
) -> Result<StatusCode, AppError> {

    // 1. Per-IP rate limit, checked before any DB lookup. A raw API key is
    // attacker-controlled and free to rotate, so limiting on it alone (as
    // this used to) lets an attacker send unlimited garbage keys — each gets
    // its own untouched bucket — and hammer the projects table below. The
    // client IP isn't something the attacker gets to reset for free.
    let ip_key = format!("ip:{}", addr.ip());
    if !state.ip_rate_limiter.check(&ip_key).await {
        return Err(AppError::TooManyRequests);
    }

    // 2. Extract API key from X-API-Key header
    let api_key = headers
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::Unauthorized("Missing X-API-Key header".into()))?;

    // 3. Validate API key
    let project = sqlx::query!(
        "SELECT id FROM projects WHERE api_key = $1",
        api_key
    )
    .fetch_optional(&state.pg_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Invalid API key".to_string()))?;

    // 4. Per-project rate limit (Redis-backed, configured via INGEST_RATE_LIMIT).
    // Keyed by the resolved project id rather than the header string, so it
    // can't be reset by anything other than the key actually rotating.
    if !state.rate_limiter.check(&project.id.to_string()).await {
        return Err(AppError::TooManyRequests);
    }

    // 5. Basic payload validation — require at least one of exception or message
    if payload.exception.is_none() && payload.message.is_none() {
        return Err(AppError::BadRequest(
            "payload must include either 'exception' or 'message'".into(),
        ));
    }

    // 6. Push to Redis queue — return immediately (fire-and-forget)
    push_job(&state.redis_pool, EventJob {
        project_id: project.id,
        payload,
    })
    .await
    .map_err(|e| AppError::BadRequest(e))?;

    tracing::info!("📥 Event queued for project: {}", project.id);

    Ok(StatusCode::ACCEPTED)
}
