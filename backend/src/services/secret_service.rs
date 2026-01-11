use crate::{
    crypto::validation::validate_encrypted_format,
    database::secret_repository::SecretRepository,
    error::AppError,
    models::secret::{CreateSecretRequest, Secret, SecretResponse},
    utils::{time::add_days_to_now, token::generate_secret_token},
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct SecretService;

impl SecretService {
    pub async fn create_secret(
        pool: &PgPool,
        creator_id: Option<Uuid>,
        request: CreateSecretRequest,
    ) -> Result<SecretResponse, AppError> {
        // Validate encrypted data format
        validate_encrypted_format(&request.encrypted_data)?;

        // Generate unique token
        let token = generate_secret_token();

        // Calculate expiration
        let expires_at = request.expires_in_days.map(add_days_to_now);

        // Create secret
        let secret = SecretRepository::create(
            pool,
            creator_id,
            request.organization_id,
            &token,
            &request.encrypted_data,
            request.encrypted_metadata.as_deref(),
            request.max_views,
            expires_at,
            request.burn_after_reading,
            false, // is_file
            None,  // file_path
            None,  // file_size
            None,  // file_mime_type
        )
        .await?;

        Ok(SecretResponse {
            id: secret.id,
            token: secret.token,
            encrypted_data: secret.encrypted_data,
            encrypted_metadata: secret.encrypted_metadata,
            max_views: secret.max_views,
            current_views: secret.current_views,
            expires_at: secret.expires_at,
            burn_after_reading: secret.burn_after_reading,
            is_file: secret.is_file,
            file_size: secret.file_size,
            file_mime_type: secret.file_mime_type,
            created_at: secret.created_at,
        })
    }

    pub async fn get_secret(
        pool: &PgPool,
        token: &str,
    ) -> Result<SecretResponse, AppError> {
        let secret = SecretRepository::find_by_token(pool, token)
            .await?
            .ok_or(AppError::NotFound)?;

        // Check expiration
        if let Some(expires_at) = secret.expires_at {
            if expires_at < Utc::now() {
                // Delete expired secret
                let _ = SecretRepository::delete_by_token(pool, token).await;
                return Err(AppError::SecretExpired);
            }
        }

        // Check max views
        if let Some(max_views) = secret.max_views {
            if secret.current_views >= max_views {
                return Err(AppError::SecretAlreadyViewed);
            }
        }

        // Increment views
        let updated_secret = SecretRepository::increment_views(pool, token)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(SecretResponse {
            id: updated_secret.id,
            token: updated_secret.token,
            encrypted_data: updated_secret.encrypted_data,
            encrypted_metadata: updated_secret.encrypted_metadata,
            max_views: updated_secret.max_views,
            current_views: updated_secret.current_views,
            expires_at: updated_secret.expires_at,
            burn_after_reading: updated_secret.burn_after_reading,
            is_file: updated_secret.is_file,
            file_size: updated_secret.file_size,
            file_mime_type: updated_secret.file_mime_type,
            created_at: updated_secret.created_at,
        })
    }

    pub async fn delete_secret(pool: &PgPool, token: &str) -> Result<(), AppError> {
        let deleted = SecretRepository::delete_by_token(pool, token).await?;
        if !deleted {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    pub async fn list_user_secrets(
        pool: &PgPool,
        creator_id: Uuid,
    ) -> Result<Vec<SecretResponse>, AppError> {
        let secrets = SecretRepository::find_by_creator(pool, creator_id).await?;

        Ok(secrets
            .into_iter()
            .map(|s| SecretResponse {
                id: s.id,
                token: s.token,
                encrypted_data: s.encrypted_data,
                encrypted_metadata: s.encrypted_metadata,
                max_views: s.max_views,
                current_views: s.current_views,
                expires_at: s.expires_at,
                burn_after_reading: s.burn_after_reading,
                is_file: s.is_file,
                file_size: s.file_size,
                file_mime_type: s.file_mime_type,
                created_at: s.created_at,
            })
            .collect())
    }
}

