use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    InternalError(String),
    TooManyRequests,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                tracing::error!("DB error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::NotFound(msg)      => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg)    => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg)  => (StatusCode::UNAUTHORIZED, msg),
            AppError::InternalError(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            AppError::TooManyRequests    => (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded".to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".into()),
            other => AppError::Database(other),
        }
    }
}
