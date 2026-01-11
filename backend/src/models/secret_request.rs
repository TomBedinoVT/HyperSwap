use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SecretRequest {
    pub id: Uuid,
    pub requester_id: Uuid,
    pub organization_id: Option<Uuid>,
    pub token: String,
    pub encrypted_prompt: String,
    pub encrypted_data: Option<String>,
    pub max_views: Option<i32>,
    pub current_views: i32,
    pub expires_at: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSecretRequestRequest {
    pub encrypted_prompt: String,
    pub expires_in_days: u32,
    pub organization_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitSecretRequest {
    pub encrypted_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretRequestResponse {
    pub id: Uuid,
    pub token: String,
    pub encrypted_prompt: Option<String>,
    pub encrypted_data: Option<String>,
    pub max_views: Option<i32>,
    pub current_views: i32,
    pub expires_at: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

