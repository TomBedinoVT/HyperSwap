use crate::models::secret_request::SecretRequest;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct SecretRequestRepository;

impl SecretRequestRepository {
    pub async fn create(
        pool: &PgPool,
        requester_id: Uuid,
        organization_id: Option<Uuid>,
        token: &str,
        encrypted_prompt: &str,
        expires_at: chrono::DateTime<Utc>,
    ) -> Result<SecretRequest, sqlx::Error> {
        let request = sqlx::query_as::<_, SecretRequest>(
            r#"
            INSERT INTO secret_requests (
                requester_id, organization_id, token, encrypted_prompt, expires_at
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id, requester_id, organization_id, token, encrypted_prompt,
                encrypted_data, max_views, current_views, expires_at, status,
                created_at, completed_at
            "#,
        )
        .bind(requester_id)
        .bind(organization_id)
        .bind(token)
        .bind(encrypted_prompt)
        .bind(expires_at)
        .fetch_one(pool)
        .await?;

        Ok(request)
    }

    pub async fn find_by_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<SecretRequest>, sqlx::Error> {
        let request = sqlx::query_as::<_, SecretRequest>(
            r#"
            SELECT
                id, requester_id, organization_id, token, encrypted_prompt,
                encrypted_data, max_views, current_views, expires_at, status,
                created_at, completed_at
            FROM secret_requests
            WHERE token = $1
            "#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await?;

        Ok(request)
    }

    pub async fn submit_secret(
        pool: &PgPool,
        token: &str,
        encrypted_data: &str,
    ) -> Result<Option<SecretRequest>, sqlx::Error> {
        let request = sqlx::query_as::<_, SecretRequest>(
            r#"
            UPDATE secret_requests
            SET encrypted_data = $1,
                status = 'completed',
                completed_at = NOW(),
                current_views = current_views + 1
            WHERE token = $2 AND status = 'pending'
            RETURNING
                id, requester_id, organization_id, token, encrypted_prompt,
                encrypted_data, max_views, current_views, expires_at, status,
                created_at, completed_at
            "#,
        )
        .bind(encrypted_data)
        .bind(token)
        .fetch_optional(pool)
        .await?;

        Ok(request)
    }

    pub async fn find_by_requester(
        pool: &PgPool,
        requester_id: Uuid,
    ) -> Result<Vec<SecretRequest>, sqlx::Error> {
        let requests = sqlx::query_as::<_, SecretRequest>(
            r#"
            SELECT
                id, requester_id, organization_id, token, encrypted_prompt,
                encrypted_data, max_views, current_views, expires_at, status,
                created_at, completed_at
            FROM secret_requests
            WHERE requester_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(requester_id)
        .fetch_all(pool)
        .await?;

        Ok(requests)
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<SecretRequest>, sqlx::Error> {
        let request = sqlx::query_as::<_, SecretRequest>(
            r#"
            SELECT
                id, requester_id, organization_id, token, encrypted_prompt,
                encrypted_data, max_views, current_views, expires_at, status,
                created_at, completed_at
            FROM secret_requests
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(request)
    }

    pub async fn delete_by_id(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM secret_requests
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
