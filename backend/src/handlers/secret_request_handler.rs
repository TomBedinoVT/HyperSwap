use axum::{extract::Path, Json};
use uuid::Uuid;

use crate::{
    error::AppError,
    middleware::auth::extract_user_id,
    models::secret_request::{
        CreateSecretRequestRequest, SecretRequestResponse, SubmitSecretRequest,
    },
    services::secret_request_service::SecretRequestService,
    AppState,
};
use axum::extract::{Request, State};

pub async fn create_secret_request(
    State(state): State<AppState>,
    request: Request,
    Json(payload): Json<CreateSecretRequestRequest>,
) -> Result<Json<SecretRequestResponse>, AppError> {
    let requester_id = extract_user_id(&request)
        .ok_or(AppError::Unauthorized)?;

    let secret_request = SecretRequestService::create_request(
        &state.db.pool,
        requester_id,
        payload,
    )
    .await?;

    Ok(Json(secret_request))
}

pub async fn get_secret_request_for_client(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<Json<SecretRequestResponse>, AppError> {
    let secret_request = SecretRequestService::get_request_for_client(
        &state.db.pool,
        &token,
    )
    .await?;

    Ok(Json(secret_request))
}

pub async fn submit_secret(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Json(payload): Json<SubmitSecretRequest>,
) -> Result<axum::http::StatusCode, AppError> {
    SecretRequestService::submit_secret(
        &state.db.pool,
        &token,
        payload,
    )
    .await?;

    Ok(axum::http::StatusCode::OK)
}

pub async fn list_secret_requests(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<Vec<SecretRequestResponse>>, AppError> {
    let requester_id = extract_user_id(&request)
        .ok_or(AppError::Unauthorized)?;

    let requests = SecretRequestService::list_user_requests(
        &state.db.pool,
        requester_id,
    )
    .await?;

    Ok(Json(requests))
}

pub async fn get_secret_request(
    State(state): State<AppState>,
    request: Request,
    Path(id): Path<Uuid>,
) -> Result<Json<SecretRequestResponse>, AppError> {
    let requester_id = extract_user_id(&request)
        .ok_or(AppError::Unauthorized)?;

    let secret_request = SecretRequestService::get_request_for_requester(
        &state.db.pool,
        requester_id,
        id,
    )
    .await?;

    Ok(Json(secret_request))
}

pub async fn delete_secret_request(
    State(state): State<AppState>,
    request: Request,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    let requester_id = extract_user_id(&request)
        .ok_or(AppError::Unauthorized)?;

    SecretRequestService::delete_request(
        &state.db.pool,
        requester_id,
        id,
    )
    .await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

