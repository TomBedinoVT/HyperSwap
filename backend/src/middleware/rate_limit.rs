use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use std::sync::Arc;

use crate::error::AppError;

pub struct RateLimitConfig {
    pub max_requests: i32,
    pub window_minutes: i64,
}

pub async fn rate_limit_middleware(
    State(pool): State<Arc<PgPool>>,
    State(config): State<RateLimitConfig>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let identifier = get_identifier(&request);
    let action = get_action(&request);

    // Check rate limit
    let count = sqlx::query!(
        r#"
        SELECT count FROM rate_limits
        WHERE identifier = $1 AND action = $2 AND expires_at > NOW()
        "#,
        identifier,
        action
    )
    .fetch_optional(pool.as_ref())
    .await?
    .map(|r| r.count)
    .unwrap_or(0);

    if count >= config.max_requests {
        return Err(AppError::RateLimitExceeded);
    }

    // Increment or create rate limit record
    let window_start = Utc::now();
    let expires_at = window_start + Duration::minutes(config.window_minutes);

    sqlx::query!(
        r#"
        INSERT INTO rate_limits (identifier, action, count, window_start, expires_at)
        VALUES ($1, $2, 1, $3, $4)
        ON CONFLICT (identifier, action) DO UPDATE
        SET count = rate_limits.count + 1
        WHERE rate_limits.expires_at > NOW()
        "#,
        identifier,
        action,
        window_start,
        expires_at
    )
    .execute(pool.as_ref())
    .await?;

    Ok(next.run(request).await)
}

fn get_identifier(request: &Request) -> String {
    // Try to get user_id from extensions first
    if let Some(user_id) = request.extensions().get::<uuid::Uuid>() {
        return format!("user:{}", user_id);
    }

    // Fall back to IP address
    request
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .map(|s| format!("ip:{}", s.split(',').next().unwrap_or("unknown")))
        .unwrap_or_else(|| "ip:unknown".to_string())
}

fn get_action(request: &Request) -> String {
    let method = request.method().as_str();
    let path = request.uri().path();

    if path.starts_with("/api/secrets") {
        format!("secret_{}", method.to_lowercase())
    } else if path.starts_with("/api/auth") {
        format!("auth_{}", method.to_lowercase())
    } else {
        format!("{}_{}", method.to_lowercase(), path.replace('/', "_"))
    }
}

