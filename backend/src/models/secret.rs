use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Secret {
    pub id: Uuid,
    pub creator_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub token: String,
    pub encrypted_data: String,
    pub encrypted_metadata: Option<String>,
    pub max_views: Option<i32>,
    pub current_views: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub burn_after_reading: bool,
    pub is_file: bool,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub file_mime_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSecretRequest {
    pub encrypted_data: String,
    pub encrypted_metadata: Option<String>,
    pub max_views: Option<i32>,
    pub expires_in_days: Option<u32>,
    pub burn_after_reading: bool,
    pub organization_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretResponse {
    pub id: Uuid,
    pub token: String,
    pub encrypted_data: String,
    pub encrypted_metadata: Option<String>,
    pub max_views: Option<i32>,
    pub current_views: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub burn_after_reading: bool,
    pub is_file: bool,
    pub file_size: Option<i64>,
    pub file_mime_type: Option<String>,
    pub created_at: DateTime<Utc>,
}

