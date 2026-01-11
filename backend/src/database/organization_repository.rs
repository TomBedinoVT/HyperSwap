use crate::models::organization::{Organization, OrganizationMember};
use sqlx::PgPool;
use uuid::Uuid;

pub struct OrganizationRepository;

impl OrganizationRepository {
    pub async fn create(
        pool: &PgPool,
        name: &str,
        slug: &str,
    ) -> Result<Organization, sqlx::Error> {
        let org = sqlx::query_as::<_, Organization>(
            r#"
            INSERT INTO organizations (name, slug)
            VALUES ($1, $2)
            RETURNING id, name, slug, created_at
            "#,
        )
        .bind(name)
        .bind(slug)
        .fetch_one(pool)
        .await?;

        Ok(org)
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<Organization>, sqlx::Error> {
        let org = sqlx::query_as::<_, Organization>(
            r#"
            SELECT id, name, slug, created_at
            FROM organizations
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(org)
    }

    pub async fn find_by_slug(
        pool: &PgPool,
        slug: &str,
    ) -> Result<Option<Organization>, sqlx::Error> {
        let org = sqlx::query_as::<_, Organization>(
            r#"
            SELECT id, name, slug, created_at
            FROM organizations
            WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(pool)
        .await?;

        Ok(org)
    }

    pub async fn find_by_user(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<Organization>, sqlx::Error> {
        let orgs = sqlx::query_as::<_, Organization>(
            r#"
            SELECT o.id, o.name, o.slug, o.created_at
            FROM organizations o
            INNER JOIN organization_members om ON o.id = om.organization_id
            WHERE om.user_id = $1
            ORDER BY o.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(orgs)
    }

    pub async fn add_member(
        pool: &PgPool,
        organization_id: Uuid,
        user_id: Uuid,
        role: &str,
    ) -> Result<OrganizationMember, sqlx::Error> {
        let member = sqlx::query_as::<_, OrganizationMember>(
            r#"
            INSERT INTO organization_members (organization_id, user_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT (organization_id, user_id) DO UPDATE SET role = $3
            RETURNING id, organization_id, user_id, role, created_at
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .bind(role)
        .fetch_one(pool)
        .await?;

        Ok(member)
    }

    pub async fn remove_member(
        pool: &PgPool,
        organization_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM organization_members
            WHERE organization_id = $1 AND user_id = $2
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_members(
        pool: &PgPool,
        organization_id: Uuid,
    ) -> Result<Vec<OrganizationMember>, sqlx::Error> {
        let members = sqlx::query_as::<_, OrganizationMember>(
            r#"
            SELECT id, organization_id, user_id, role, created_at
            FROM organization_members
            WHERE organization_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(organization_id)
        .fetch_all(pool)
        .await?;

        Ok(members)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM organizations
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
