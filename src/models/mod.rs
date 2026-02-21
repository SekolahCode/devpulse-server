use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id:         Uuid,
    pub name:       String,
    pub api_key:    String,
    pub platform:   Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngestPayload {
    pub platform:    Option<String>,
    pub level:       Option<String>,       // "error" | "warning" | "info"
    pub environment: Option<String>,       // "production" | "staging" | "development" | …
    pub exception:   Option<ExceptionInfo>,
    pub message:     Option<String>,
    pub context:     Option<serde_json::Value>,
    pub user:        Option<serde_json::Value>,
    pub request:     Option<serde_json::Value>,
    pub timestamp:   Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionInfo {
    pub r#type:     Option<String>,   // exception class name
    pub message:    String,
    pub stacktrace: Option<Vec<StackFrame>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackFrame {
    pub file:     Option<String>,
    pub line:     Option<u32>,
    pub function: Option<String>,
    pub context:  Option<String>,
}
