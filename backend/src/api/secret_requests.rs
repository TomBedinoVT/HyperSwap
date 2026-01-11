use axum::{routing::{delete, get, post}, Router};
use crate::{handlers::secret_request_handler, AppState};

pub fn create_secret_requests_router() -> Router<AppState> {
    Router::new()
        .route("/", post(secret_request_handler::create_secret_request))
        .route("/", get(secret_request_handler::list_secret_requests))
        .route("/:token", get(secret_request_handler::get_secret_request_for_client))
        .route("/:token/submit", post(secret_request_handler::submit_secret))
        .route("/:id", get(secret_request_handler::get_secret_request))
        .route("/:id", delete(secret_request_handler::delete_secret_request))
}

