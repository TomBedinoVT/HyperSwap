use axum::{
    extract::{Path, Request, State},
    Json,
};

use crate::{
    error::AppError,
    middleware::auth::extract_user_id,
    models::secret::{CreateSecretRequest, SecretResponse},
    services::secret_service::SecretService,
    AppState,
};

pub async fn create_secret(
    State(state): State<AppState>,
    request: Request,
    Json(payload): Json<CreateSecretRequest>,
) -> Result<Json<SecretResponse>, AppError> {
    let creator_id = extract_user_id(&request);
    
    let secret = SecretService::create_secret(
        &state.db.pool,
        creator_id,
        payload,
    )
    .await?;

    Ok(Json(secret))
}

pub async fn get_secret(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<Json<SecretResponse>, AppError> {
    let secret = SecretService::get_secret(&state.db.pool, &token).await?;

    Ok(Json(secret))
}

pub async fn delete_secret(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<axum::http::StatusCode, AppError> {
    SecretService::delete_secret(&state.db.pool, &token).await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub async fn list_secrets(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<Vec<SecretResponse>>, AppError> {
    let user_id = extract_user_id(&request)
        .ok_or(AppError::Unauthorized)?;

    let secrets = SecretService::list_user_secrets(&state.db.pool, user_id).await?;

    Ok(Json(secrets))
}

