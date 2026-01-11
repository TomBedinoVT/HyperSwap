use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, avatar_url, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, avatar_url, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn create(
        pool: &PgPool,
        email: &str,
        name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, name, avatar_url)
            VALUES ($1, $2, $3)
            RETURNING id, email, name, avatar_url, created_at, updated_at
            "#,
        )
        .bind(email)
        .bind(name)
        .bind(avatar_url)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_or_create_by_oauth(
        pool: &PgPool,
        provider: &str,
        provider_user_id: &str,
        email: &str,
        name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        // Try to find existing OAuth provider link
        let existing_link = sqlx::query!(
            r#"
            SELECT user_id FROM oauth_providers
            WHERE provider = $1 AND provider_user_id = $2
            "#,
            provider,
            provider_user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(link) = existing_link {
            // User exists, return it
            return Self::find_by_id(pool, link.user_id).await?
                .ok_or_else(|| sqlx::Error::RowNotFound);
        }

        // Check if user exists by email
        let existing_user = Self::find_by_email(pool, email).await?;
        
        let user = if let Some(user) = existing_user {
            // User exists, link OAuth provider
            sqlx::query!(
                r#"
                INSERT INTO oauth_providers (user_id, provider, provider_user_id)
                VALUES ($1, $2, $3)
                ON CONFLICT (provider, provider_user_id) DO NOTHING
                "#,
                user.id,
                provider,
                provider_user_id
            )
            .execute(pool)
            .await?;
            
            user
        } else {
            // Create new user
            let new_user = Self::create(pool, email, name, avatar_url).await?;
            
            // Link OAuth provider
            sqlx::query!(
                r#"
                INSERT INTO oauth_providers (user_id, provider, provider_user_id)
                VALUES ($1, $2, $3)
                "#,
                new_user.id,
                provider,
                provider_user_id
            )
            .execute(pool)
            .await?;
            
            new_user
        };

        Ok(user)
    }
}

