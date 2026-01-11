use crate::{
    database::organization_repository::OrganizationRepository,
    error::AppError,
    models::organization::{AddMemberRequest, CreateOrganizationRequest, Organization, OrganizationMember},
    utils::token::generate_slug,
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct OrganizationService;

impl OrganizationService {
    pub async fn create_organization(
        pool: &PgPool,
        creator_id: Uuid,
        request: CreateOrganizationRequest,
    ) -> Result<(Organization, OrganizationMember), AppError> {
        // Generate slug
        let slug = generate_slug(&request.name);

        // Check if slug exists
        if OrganizationRepository::find_by_slug(pool, &slug).await?.is_some() {
            return Err(AppError::Validation(format!(
                "Organization with slug '{}' already exists",
                slug
            )));
        }

        // Create organization
        let org = OrganizationRepository::create(pool, &request.name, &slug).await?;

        // Add creator as owner
        let member = OrganizationRepository::add_member(pool, org.id, creator_id, "owner")
            .await?;

        Ok((org, member))
    }

    pub async fn get_organization(
        pool: &PgPool,
        org_id: Uuid,
    ) -> Result<Organization, AppError> {
        let org = OrganizationRepository::find_by_id(pool, org_id)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(org)
    }

    pub async fn list_user_organizations(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<Organization>, AppError> {
        let orgs = OrganizationRepository::find_by_user(pool, user_id).await?;
        Ok(orgs)
    }

    pub async fn add_member(
        pool: &PgPool,
        org_id: Uuid,
        request: AddMemberRequest,
    ) -> Result<OrganizationMember, AppError> {
        // Verify organization exists
        OrganizationRepository::find_by_id(pool, org_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let member = OrganizationRepository::add_member(pool, org_id, request.user_id, &request.role)
            .await?;

        Ok(member)
    }

    pub async fn remove_member(
        pool: &PgPool,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let removed = OrganizationRepository::remove_member(pool, org_id, user_id).await?;
        if !removed {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    pub async fn get_members(
        pool: &PgPool,
        org_id: Uuid,
    ) -> Result<Vec<OrganizationMember>, AppError> {
        let members = OrganizationRepository::get_members(pool, org_id).await?;
        Ok(members)
    }

    pub async fn delete_organization(
        pool: &PgPool,
        org_id: Uuid,
    ) -> Result<(), AppError> {
        let deleted = OrganizationRepository::delete(pool, org_id).await?;
        if !deleted {
            return Err(AppError::NotFound);
        }
        Ok(())
    }
}

