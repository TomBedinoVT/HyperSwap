use axum::{routing::get, Router};
use crate::{handlers::auth_handler, AppState};

pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        .route("/oauth/google", axum::routing::get(auth_handler::oauth_google))
        .route("/oauth/google/callback", axum::routing::get(auth_handler::oauth_google_callback))
        .route("/me", get(auth_handler::get_me))
}

