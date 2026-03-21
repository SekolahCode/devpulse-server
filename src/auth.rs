use axum::{extract::Request, middleware::Next, response::{IntoResponse, Response}};
use crate::errors::AppError;

/// Tower middleware — checks `Authorization: Bearer <ADMIN_TOKEN>` on every
/// request that passes through it.  If `ADMIN_TOKEN` is not set or is empty,
/// all requests are rejected (fail-closed).
pub async fn require_admin_token(request: Request, next: Next) -> Response {
    let token = match std::env::var("ADMIN_TOKEN").ok().filter(|t| !t.is_empty()) {
        Some(t) => t,
        None => {
            return AppError::Unauthorized(
                "Server is not configured with an ADMIN_TOKEN".into(),
            )
            .into_response();
        }
    };

    let provided = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    if provided != Some(token.as_str()) {
        return AppError::Unauthorized("Missing or invalid Authorization token".into())
            .into_response();
    }

    next.run(request).await
}
