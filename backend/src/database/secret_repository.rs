use crate::models::secret::Secret;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct SecretRepository;

impl SecretRepository {
    pub async fn create(
        pool: &PgPool,
        creator_id: Option<Uuid>,
        organization_id: Option<Uuid>,
        token: &str,
        encrypted_data: &str,
        encrypted_metadata: Option<&str>,
        max_views: Option<i32>,
        expires_at: Option<chrono::DateTime<Utc>>,
        burn_after_reading: bool,
        is_file: bool,
        file_path: Option<&str>,
        file_size: Option<i64>,
        file_mime_type: Option<&str>,
    ) -> Result<Secret, sqlx::Error> {
        let secret = sqlx::query_as::<_, Secret>(
            r#"
            INSERT INTO secrets (
                creator_id, organization_id, token, encrypted_data, encrypted_metadata,
                max_views, expires_at, burn_after_reading, is_file,
                file_path, file_size, file_mime_type
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING
                id, creator_id, organization_id, token, encrypted_data, encrypted_metadata,
                max_views, current_views, expires_at, burn_after_reading, is_file,
                file_path, file_size, file_mime_type, created_at, last_accessed_at
            "#,
        )
        .bind(creator_id)
        .bind(organization_id)
        .bind(token)
        .bind(encrypted_data)
        .bind(encrypted_metadata)
        .bind(max_views)
        .bind(expires_at)
        .bind(burn_after_reading)
        .bind(is_file)
        .bind(file_path)
        .bind(file_size)
        .bind(file_mime_type)
        .fetch_one(pool)
        .await?;

        Ok(secret)
    }

    pub async fn find_by_token(pool: &PgPool, token: &str) -> Result<Option<Secret>, sqlx::Error> {
        let secret = sqlx::query_as::<_, Secret>(
            r#"
            SELECT
                id, creator_id, organization_id, token, encrypted_data, encrypted_metadata,
                max_views, current_views, expires_at, burn_after_reading, is_file,
                file_path, file_size, file_mime_type, created_at, last_accessed_at
            FROM secrets
            WHERE token = $1
            "#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await?;

        Ok(secret)
    }

    pub async fn increment_views(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<Secret>, sqlx::Error> {
        let secret = sqlx::query_as::<_, Secret>(
            r#"
            UPDATE secrets
            SET current_views = current_views + 1,
                last_accessed_at = NOW()
            WHERE token = $1
            RETURNING
                id, creator_id, organization_id, token, encrypted_data, encrypted_metadata,
                max_views, current_views, expires_at, burn_after_reading, is_file,
                file_path, file_size, file_mime_type, created_at, last_accessed_at
            "#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await?;

        Ok(secret)
    }

    pub async fn delete_by_token(pool: &PgPool, token: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM secrets
            WHERE token = $1
            "#,
            token
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find_by_creator(
        pool: &PgPool,
        creator_id: Uuid,
    ) -> Result<Vec<Secret>, sqlx::Error> {
        let secrets = sqlx::query_as::<_, Secret>(
            r#"
            SELECT
                id, creator_id, organization_id, token, encrypted_data, encrypted_metadata,
                max_views, current_views, expires_at, burn_after_reading, is_file,
                file_path, file_size, file_mime_type, created_at, last_accessed_at
            FROM secrets
            WHERE creator_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(creator_id)
        .fetch_all(pool)
        .await?;

        Ok(secrets)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Secret>, sqlx::Error> {
        let secret = sqlx::query_as::<_, Secret>(
            r#"
            SELECT
                id, creator_id, organization_id, token, encrypted_data, encrypted_metadata,
                max_views, current_views, expires_at, burn_after_reading, is_file,
                file_path, file_size, file_mime_type, created_at, last_accessed_at
            FROM secrets
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(secret)
    }
}

