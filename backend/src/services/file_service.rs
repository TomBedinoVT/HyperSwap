use crate::{
    crypto::validation::validate_encrypted_format,
    database::secret_repository::SecretRepository,
    error::AppError,
    storage::s3_client::S3Client,
    utils::{time::add_days_to_now, token::generate_secret_token},
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct FileService;

impl FileService {
    pub async fn upload_file(
        pool: &PgPool,
        s3_client: &S3Client,
        creator_id: Option<Uuid>,
        organization_id: Option<Uuid>,
        encrypted_data: String,
        encrypted_metadata: Option<String>,
        file_size: i64,
        file_mime_type: String,
        max_views: Option<i32>,
        expires_in_days: Option<u32>,
        burn_after_reading: bool,
    ) -> Result<String, AppError> {
        // Validate encrypted data format
        validate_encrypted_format(&encrypted_data)?;

        // Generate unique token and file path
        let token = generate_secret_token();
        let file_path = format!("files/{}", token);

        // Upload to S3
        s3_client
            .upload(
                &file_path,
                encrypted_data.as_bytes().to_vec(),
                Some("application/octet-stream"),
            )
            .await
            .map_err(|e| AppError::S3(e.to_string()))?;

        // Calculate expiration
        let expires_at = expires_in_days.map(add_days_to_now);

        // Create secret record
        SecretRepository::create(
            pool,
            creator_id,
            organization_id,
            &token,
            &encrypted_data, // Store encrypted metadata in DB too
            encrypted_metadata.as_deref(),
            max_views,
            expires_at,
            burn_after_reading,
            true, // is_file
            Some(&file_path),
            Some(file_size),
            Some(&file_mime_type),
        )
        .await?;

        Ok(token)
    }

    pub async fn download_file(
        pool: &PgPool,
        s3_client: &S3Client,
        token: &str,
    ) -> Result<(Vec<u8>, String), AppError> {
        let secret = SecretRepository::find_by_token(pool, token)
            .await?
            .ok_or(AppError::NotFound)?;

        if !secret.is_file {
            return Err(AppError::Validation("Not a file secret".to_string()));
        }

        let file_path = secret.file_path.ok_or(AppError::Internal(
            "File path not found".to_string(),
        ))?;

        // Download from S3
        let data = s3_client
            .download(&file_path)
            .await
            .map_err(|e| AppError::S3(e.to_string()))?;

        let mime_type = secret.file_mime_type.unwrap_or_else(|| "application/octet-stream".to_string());

        // Increment views
        let _ = SecretRepository::increment_views(pool, token).await;

        Ok((data, mime_type))
    }

    pub async fn delete_file(
        pool: &PgPool,
        s3_client: &S3Client,
        token: &str,
    ) -> Result<(), AppError> {
        let secret = SecretRepository::find_by_token(pool, token)
            .await?
            .ok_or(AppError::NotFound)?;

        if let Some(file_path) = secret.file_path {
            // Delete from S3
            let _ = s3_client.delete(&file_path).await;
        }

        // Delete from DB
        SecretRepository::delete_by_token(pool, token).await?;

        Ok(())
    }
}

