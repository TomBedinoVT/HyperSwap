use crate::{
    config::Config,
    database::user_repository::UserRepository,
    error::AppError,
    models::user::User,
    utils::token::generate_secret_token,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,
    pub iat: usize,
}

pub struct AuthService;

impl AuthService {
    pub fn generate_jwt(user_id: Uuid, config: &Config) -> Result<String, AppError> {
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::minutes(config.jwt_expiration_minutes as i64);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_ref()),
        )?;

        Ok(token)
    }

    pub async fn find_or_create_user_by_oauth(
        pool: &PgPool,
        provider: &str,
        provider_user_id: &str,
        email: &str,
        name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> Result<User, AppError> {
        let user = UserRepository::find_or_create_by_oauth(
            pool,
            provider,
            provider_user_id,
            email,
            name,
            avatar_url,
        )
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
        let user = UserRepository::find_by_id(pool, id).await?;
        Ok(user)
    }
}

