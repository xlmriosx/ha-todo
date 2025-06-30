use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::response::ApiResponse;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    ValidationError(String),
    #[allow(dead_code)]
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred".to_string())
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalError(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        (status, Json(ApiResponse::<()>::error(&message))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(format!("Validation failed: {}", err))
    }
}