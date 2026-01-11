use serde::Deserialize;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expiration_minutes: u64,
    pub s3_endpoint: Option<String>,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key_id: String,
    pub s3_secret_access_key: String,
    pub oauth_google_client_id: Option<String>,
    pub oauth_google_client_secret: Option<String>,
    pub oauth_microsoft_client_id: Option<String>,
    pub oauth_microsoft_client_secret: Option<String>,
    pub oauth_github_client_id: Option<String>,
    pub oauth_github_client_secret: Option<String>,
    pub frontend_url: String,
    pub environment: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiration_minutes: env::var("JWT_EXPIRATION_MINUTES")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            s3_endpoint: env::var("S3_ENDPOINT").ok(),
            s3_region: env::var("S3_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),
            s3_bucket: env::var("S3_BUCKET")
                .expect("S3_BUCKET must be set"),
            s3_access_key_id: env::var("S3_ACCESS_KEY_ID")
                .expect("S3_ACCESS_KEY_ID must be set"),
            s3_secret_access_key: env::var("S3_SECRET_ACCESS_KEY")
                .expect("S3_SECRET_ACCESS_KEY must be set"),
            oauth_google_client_id: env::var("OAUTH_GOOGLE_CLIENT_ID").ok(),
            oauth_google_client_secret: env::var("OAUTH_GOOGLE_CLIENT_SECRET").ok(),
            oauth_microsoft_client_id: env::var("OAUTH_MICROSOFT_CLIENT_ID").ok(),
            oauth_microsoft_client_secret: env::var("OAUTH_MICROSOFT_CLIENT_SECRET").ok(),
            oauth_github_client_id: env::var("OAUTH_GITHUB_CLIENT_ID").ok(),
            oauth_github_client_secret: env::var("OAUTH_GITHUB_CLIENT_SECRET").ok(),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}

