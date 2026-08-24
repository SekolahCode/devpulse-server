/// Tower middleware that attaches a UUID trace ID to every request.
///
/// The ID is sourced from the incoming `X-Trace-Id` header when present
/// (so callers can propagate their own trace), otherwise a new v4 UUID is
/// generated.  The resolved ID is:
///
/// 1. Inserted into the current `tracing` span as a `trace_id` field so every
///    log line emitted while handling the request carries it.
/// 2. Written back to the response as `X-Trace-Id` so clients can correlate
///    their own logs.
use axum::{
    body::Body,
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use tracing::Instrument;
use uuid::Uuid;

pub async fn trace_id_middleware(request: Request<Body>, next: Next) -> Response {
    // Reuse caller-supplied trace ID or generate a fresh one.
    let trace_id = request
        .headers()
        .get("X-Trace-Id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Use .instrument() instead of span.enter() so the span is correctly
    // propagated across .await points in Tokio's multi-threaded runtime.
    let span = tracing::info_span!("request", trace_id = %trace_id);
    let mut response = next.run(request).instrument(span).await;

    // Echo the trace ID back to the caller (and JS can read it via expose_headers).
    if let Ok(value) = HeaderValue::from_str(&trace_id) {
        response.headers_mut().insert("X-Trace-Id", value);
    }

    response
}
