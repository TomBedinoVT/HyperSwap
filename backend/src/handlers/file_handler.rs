use axum::{
    body::Bytes,
    extract::{Path, Request, State},
    http::{header, StatusCode},
    response::Response,
    Json,
};

use crate::{
    error::AppError,
    middleware::auth::extract_user_id,
    services::file_service::FileService,
    AppState,
};

pub async fn upload_file(
    State(state): State<AppState>,
    request: Request,
    body: Bytes,
) -> Result<Json<serde_json::Value>, AppError> {
    // TODO: Parse multipart form data
    // For now, this is a placeholder
    let creator_id = extract_user_id(&request);
    
    // This would need proper multipart parsing
    Err(AppError::Internal("File upload not fully implemented".to_string()))
}

pub async fn download_file(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<Response, AppError> {
    let (data, mime_type) = FileService::download_file(
        &state.db.pool,
        &state.s3_client,
        &token,
    )
    .await?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime_type)
        .header(header::CONTENT_DISPOSITION, "attachment")
        .body(axum::body::Body::from(data))
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(response)
}

pub async fn delete_file(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<StatusCode, AppError> {
    FileService::delete_file(
        &state.db.pool,
        &state.s3_client,
        &token,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

