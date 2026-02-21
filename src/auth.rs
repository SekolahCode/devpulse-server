use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;

/// Tower middleware — checks `Authorization: Bearer <ADMIN_TOKEN>` on every
/// request that passes through it.  If `ADMIN_TOKEN` is not set in the
/// environment the middleware is a no-op (dev-friendly default).
pub async fn require_admin_token(request: Request, next: Next) -> Response {
    let token = std::env::var("ADMIN_TOKEN").unwrap_or_default();

    if !token.is_empty() {
        let provided = request
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        if provided != Some(token.as_str()) {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Missing or invalid Authorization token" })),
            )
                .into_response();
        }
    }

    next.run(request).await
}
