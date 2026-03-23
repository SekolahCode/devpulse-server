use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct IngestPayload {
    pub platform:    Option<String>,
    pub level:       Option<String>,       // "error" | "warning" | "info"
    pub environment: Option<String>,       // "production" | "staging" | "development" | …
    pub release:     Option<String>,       // app version / git tag
    pub exception:   Option<ExceptionInfo>,
    pub message:     Option<String>,
    pub context:     Option<serde_json::Value>,
    pub user:        Option<serde_json::Value>,
    pub request:     Option<serde_json::Value>,
    pub breadcrumbs: Option<Vec<Breadcrumb>>,
    pub timestamp:   Option<String>,
    /// Captures all SDK-specific extra fields (e.g. Laravel's command, exit_code,
    /// laravel version, php version) so they are preserved in the stored payload
    /// and available in the dashboard without being silently dropped.
    #[serde(flatten)]
    pub extra:       HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionInfo {
    pub r#type:     Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Breadcrumb {
    pub timestamp: Option<String>,
    pub category:  Option<String>,   // "navigation", "http", "ui.click", "console", …
    pub message:   Option<String>,
    pub level:     Option<String>,
    pub data:      Option<serde_json::Value>,
}
