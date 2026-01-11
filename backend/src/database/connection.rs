use sqlx::{PgPool, Postgres};
use std::time::Duration;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect_with(
            database_url
                .parse()
                .map_err(|e| sqlx::Error::Configuration(format!("Invalid database URL: {}", e).into()))?,
        )
        .await?;

        // Configure pool
        pool.set_max_connections(10);
        pool.set_acquire_timeout(Duration::from_secs(30));

        Ok(Database { pool })
    }
}

