use axum::{routing::{delete, get, post}, Router};
use crate::{handlers::secret_handler, AppState};

pub fn create_secrets_router() -> Router<AppState> {
    Router::new()
        .route("/", post(secret_handler::create_secret))
        .route("/", get(secret_handler::list_secrets))
        .route("/:token", get(secret_handler::get_secret))
        .route("/:token", delete(secret_handler::delete_secret))
}

