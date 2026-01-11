use axum::{extract::Path, Json};
use uuid::Uuid;

use crate::{
    error::AppError,
    middleware::auth::extract_user_id,
    models::organization::{AddMemberRequest, CreateOrganizationRequest, Organization, OrganizationMember},
    services::organization_service::OrganizationService,
    AppState,
};
use axum::extract::{Request, State};

pub async fn create_organization(
    State(state): State<AppState>,
    request: Request,
    Json(payload): Json<CreateOrganizationRequest>,
) -> Result<Json<Organization>, AppError> {
    let creator_id = extract_user_id(&request)
        .ok_or(AppError::Unauthorized)?;

    let (org, _member) = OrganizationService::create_organization(
        &state.db.pool,
        creator_id,
        payload,
    )
    .await?;

    Ok(Json(org))
}

pub async fn get_organization(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Organization>, AppError> {
    let org = OrganizationService::get_organization(&state.db.pool, id).await?;

    Ok(Json(org))
}

pub async fn list_organizations(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<Vec<Organization>>, AppError> {
    let user_id = extract_user_id(&request)
        .ok_or(AppError::Unauthorized)?;

    let orgs = OrganizationService::list_user_organizations(&state.db.pool, user_id).await?;

    Ok(Json(orgs))
}

pub async fn add_member(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<AddMemberRequest>,
) -> Result<Json<OrganizationMember>, AppError> {
    let member = OrganizationService::add_member(
        &state.db.pool,
        id,
        payload,
    )
    .await?;

    Ok(Json(member))
}

pub async fn remove_member(
    State(state): State<AppState>,
    Path((org_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<axum::http::StatusCode, AppError> {
    OrganizationService::remove_member(
        &state.db.pool,
        org_id,
        user_id,
    )
    .await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub async fn get_members(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<OrganizationMember>>, AppError> {
    let members = OrganizationService::get_members(&state.db.pool, id).await?;

    Ok(Json(members))
}

pub async fn delete_organization(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, AppError> {
    OrganizationService::delete_organization(&state.db.pool, id).await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

