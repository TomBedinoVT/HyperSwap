use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::Config, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,
    pub iat: usize,
}

pub async fn auth_middleware(
    State(config): State<Config>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }

    let token = &auth_header[7..];

    let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_ref());
    let validation = Validation::default();

    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|_| AppError::Unauthorized)?;

    let user_id: Uuid = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| AppError::Unauthorized)?;

    // Add user_id to request extensions
    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}

pub fn extract_user_id(request: &Request) -> Option<Uuid> {
    request.extensions().get::<Uuid>().copied()
}

