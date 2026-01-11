use axum::{routing::{delete, get, post}, Router};
use crate::{handlers::organization_handler, AppState};

pub fn create_organizations_router() -> Router<AppState> {
    Router::new()
        .route("/", post(organization_handler::create_organization))
        .route("/", get(organization_handler::list_organizations))
        .route("/:id", get(organization_handler::get_organization))
        .route("/:id", delete(organization_handler::delete_organization))
        .route("/:id/members", get(organization_handler::get_members))
        .route("/:id/members", post(organization_handler::add_member))
        .route("/:id/members/:user_id", delete(organization_handler::remove_member))
}

