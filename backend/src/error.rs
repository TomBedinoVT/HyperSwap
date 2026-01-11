use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("OAuth error: {0}")]
    OAuth(String),

    #[error("S3 error: {0}")]
    S3(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Secret expired")]
    SecretExpired,

    #[error("Secret already viewed")]
    SecretAlreadyViewed,

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::Config(e) => {
                tracing::error!("Configuration error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error".to_string())
            }
            AppError::Jwt(e) => {
                tracing::warn!("JWT error: {}", e);
                (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
            }
            AppError::OAuth(msg) => {
                tracing::warn!("OAuth error: {}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::S3(msg) => {
                tracing::error!("S3 error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Storage error".to_string())
            }
            AppError::Validation(msg) => {
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "Resource not found".to_string())
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
            }
            AppError::Forbidden => {
                (StatusCode::FORBIDDEN, "Forbidden".to_string())
            }
            AppError::RateLimitExceeded => {
                (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded".to_string())
            }
            AppError::SecretExpired => {
                (StatusCode::GONE, "Secret expired".to_string())
            }
            AppError::SecretAlreadyViewed => {
                (StatusCode::GONE, "Secret already viewed".to_string())
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

