use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::AppState;

/// GET /api/ai/providers — return which AI providers are configured.
pub async fn get_providers(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "anthropic": state.anthropic_key.is_some(),
        "openai":    state.openai_key.is_some(),
        "gemini":    state.gemini_key.is_some(),
    }))
}
