use crate::{
    crypto::validation::validate_encrypted_format,
    database::secret_request_repository::SecretRequestRepository,
    error::AppError,
    models::secret_request::{
        CreateSecretRequestRequest, SecretRequest, SecretRequestResponse, SubmitSecretRequest,
    },
    utils::{time::add_days_to_now, token::generate_secret_token},
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct SecretRequestService;

impl SecretRequestService {
    pub async fn create_request(
        pool: &PgPool,
        requester_id: Uuid,
        request: CreateSecretRequestRequest,
    ) -> Result<SecretRequestResponse, AppError> {
        // Validate encrypted prompt format
        validate_encrypted_format(&request.encrypted_prompt)?;

        // Generate unique token
        let token = generate_secret_token();

        // Calculate expiration
        let expires_at = add_days_to_now(request.expires_in_days);

        // Create request
        let secret_request = SecretRequestRepository::create(
            pool,
            requester_id,
            request.organization_id,
            &token,
            &request.encrypted_prompt,
            expires_at,
        )
        .await?;

        Ok(SecretRequestResponse {
            id: secret_request.id,
            token: secret_request.token,
            encrypted_prompt: Some(secret_request.encrypted_prompt),
            encrypted_data: secret_request.encrypted_data,
            max_views: secret_request.max_views,
            current_views: secret_request.current_views,
            expires_at: secret_request.expires_at,
            status: secret_request.status,
            created_at: secret_request.created_at,
            completed_at: secret_request.completed_at,
        })
    }

    pub async fn get_request_for_client(
        pool: &PgPool,
        token: &str,
    ) -> Result<SecretRequestResponse, AppError> {
        let request = SecretRequestRepository::find_by_token(pool, token)
            .await?
            .ok_or(AppError::NotFound)?;

        // Check expiration
        if request.expires_at < Utc::now() {
            return Err(AppError::SecretExpired);
        }

        // Check status
        if request.status != "pending" {
            return Err(AppError::SecretAlreadyViewed);
        }

        Ok(SecretRequestResponse {
            id: request.id,
            token: request.token,
            encrypted_prompt: Some(request.encrypted_prompt),
            encrypted_data: None, // Don't reveal if already completed
            max_views: request.max_views,
            current_views: request.current_views,
            expires_at: request.expires_at,
            status: request.status,
            created_at: request.created_at,
            completed_at: request.completed_at,
        })
    }

    pub async fn submit_secret(
        pool: &PgPool,
        token: &str,
        submit: SubmitSecretRequest,
    ) -> Result<(), AppError> {
        // Validate encrypted data format
        validate_encrypted_format(&submit.encrypted_data)?;

        // Submit secret
        let updated = SecretRequestRepository::submit_secret(pool, token, &submit.encrypted_data)
            .await?
            .ok_or(AppError::NotFound)?;

        if updated.status != "completed" {
            return Err(AppError::Internal("Failed to submit secret".to_string()));
        }

        Ok(())
    }

    pub async fn get_request_for_requester(
        pool: &PgPool,
        requester_id: Uuid,
        request_id: Uuid,
    ) -> Result<SecretRequestResponse, AppError> {
        let request = SecretRequestRepository::find_by_id(pool, request_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // Verify ownership
        if request.requester_id != requester_id {
            return Err(AppError::Forbidden);
        }

        Ok(SecretRequestResponse {
            id: request.id,
            token: request.token,
            encrypted_prompt: Some(request.encrypted_prompt),
            encrypted_data: request.encrypted_data,
            max_views: request.max_views,
            current_views: request.current_views,
            expires_at: request.expires_at,
            status: request.status,
            created_at: request.created_at,
            completed_at: request.completed_at,
        })
    }

    pub async fn list_user_requests(
        pool: &PgPool,
        requester_id: Uuid,
    ) -> Result<Vec<SecretRequestResponse>, AppError> {
        let requests = SecretRequestRepository::find_by_requester(pool, requester_id).await?;

        Ok(requests
            .into_iter()
            .map(|r| SecretRequestResponse {
                id: r.id,
                token: r.token,
                encrypted_prompt: Some(r.encrypted_prompt),
                encrypted_data: r.encrypted_data,
                max_views: r.max_views,
                current_views: r.current_views,
                expires_at: r.expires_at,
                status: r.status,
                created_at: r.created_at,
                completed_at: r.completed_at,
            })
            .collect())
    }

    pub async fn delete_request(
        pool: &PgPool,
        requester_id: Uuid,
        request_id: Uuid,
    ) -> Result<(), AppError> {
        // Verify ownership
        let request = SecretRequestRepository::find_by_id(pool, request_id)
            .await?
            .ok_or(AppError::NotFound)?;

        if request.requester_id != requester_id {
            return Err(AppError::Forbidden);
        }

        // Delete
        let deleted = SecretRequestRepository::delete_by_id(pool, request_id).await?;
        if !deleted {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}

