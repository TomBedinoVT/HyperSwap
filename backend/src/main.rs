use axum::{
    extract::Request,
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod crypto;
mod database;
mod error;
mod handlers;
mod middleware;
mod models;
mod services;
mod storage;
mod utils;

use config::Config;
use database::connection::Database;
use error::AppError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hyperswap_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenv::dotenv().ok();
    let config = Config::from_env()?;

    // Initialize database
    let db = Database::new(&config.database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db.pool)
        .await?;

    info!("Database migrations completed");

    // Initialize S3 storage
    let s3_client = storage::s3_client::S3Client::new(&config).await?;

    // Build application state
    let app_state = AppState {
        db,
        s3_client,
        config: config.clone(),
    };

    // Build router
    let app = create_router(app_state).await?;

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_router(state: AppState) -> Result<Router<AppState>, AppError> {
    let router = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api::create_api_router().await?)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(CorsLayer::permissive()) // TODO: Make restrictive in production
                .into_inner(),
        )
        .with_state(state);

    Ok(router)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub s3_client: storage::s3_client::S3Client,
    pub config: Config,
}

