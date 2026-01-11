use axum::{
    extract::{Query, State},
    response::Redirect,
    Json,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, ClientId, ClientSecret,
    RedirectUrl, TokenResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    config::Config,
    error::AppError,
    services::auth_service::AuthService,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    code: String,
    state: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

pub async fn oauth_google(
    State(state): State<AppState>,
) -> Result<Redirect, AppError> {
    let config = &state.config;
    
    let client_id = config
        .oauth_google_client_id
        .as_ref()
        .ok_or_else(|| AppError::OAuth("Google OAuth not configured".to_string()))?;
    
    let client_secret = config
        .oauth_google_client_secret
        .as_ref()
        .ok_or_else(|| AppError::OAuth("Google OAuth not configured".to_string()))?;

    let redirect_url = format!("{}/api/auth/oauth/google/callback", config.frontend_url);
    
    let client = BasicClient::new(
        ClientId::new(client_id.clone()),
        Some(ClientSecret::new(client_secret.clone())),
        oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .map_err(|e| AppError::OAuth(e.to_string()))?,
        Some(
            oauth2::TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
                .map_err(|e| AppError::OAuth(e.to_string()))?,
        ),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).map_err(|e| AppError::OAuth(e.to_string()))?);

    let (auth_url, _csrf_token) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .url();

    Ok(Redirect::to(auth_url.as_str()))
}

pub async fn oauth_google_callback(
    State(state): State<AppState>,
    Query(query): Query<OAuthCallbackQuery>,
) -> Result<Json<AuthResponse>, AppError> {
    // TODO: Implement full OAuth flow with user info retrieval
    // This is a simplified version
    Err(AppError::OAuth("Not fully implemented".to_string()))
}

pub async fn get_me(
    State(state): State<AppState>,
    user_id: uuid::Uuid,
) -> Result<Json<UserInfo>, AppError> {
    let user = AuthService::find_user_by_id(&state.db.pool, user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(UserInfo {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        avatar_url: user.avatar_url,
    }))
}

