use axum::{routing::{delete, get, post}, Router};
use crate::{handlers::file_handler, AppState};

pub fn create_files_router() -> Router<AppState> {
    Router::new()
        .route("/upload", post(file_handler::upload_file))
        .route("/:token", get(file_handler::download_file))
        .route("/:token", delete(file_handler::delete_file))
}

