pub mod auth;
pub mod files;
pub mod organizations;
pub mod secrets;
pub mod secret_requests;

use axum::Router;
use crate::AppState;

pub async fn create_api_router() -> Result<Router<AppState>, crate::error::AppError> {
    let router = Router::new()
        .nest("/auth", auth::create_auth_router())
        .nest("/secrets", secrets::create_secrets_router())
        .nest("/secret-requests", secret_requests::create_secret_requests_router())
        .nest("/organizations", organizations::create_organizations_router())
        .nest("/files", files::create_files_router());

    Ok(router)
}

