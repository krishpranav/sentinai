use reqwest::Client;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use crate::config::Config;
use crate::error::AppError;

use crate::domain::realtime::RealtimeEvent;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub http_client: Client,
    pub tx: broadcast::Sender<RealtimeEvent>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, AppError> {
        let rust_env = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());

        let mut connect_options = PgConnectOptions::from_str(&config.database_url)
            .map_err(|e| AppError::InternalServerError(format!("Invalid DB URL: {}", e)))?;

        if rust_env == "production" {
            connect_options = connect_options
                .log_statements(tracing::log::LevelFilter::Off)
                .log_slow_statements(tracing::log::LevelFilter::Warn, Duration::from_millis(200));
        } else {
            connect_options = connect_options
                .log_statements(tracing::log::LevelFilter::Debug)
                .log_slow_statements(tracing::log::LevelFilter::Warn, Duration::from_millis(200));
        }

        let db = PgPoolOptions::new()
            .connect_with(connect_options)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Migration failed: {}", e)))?;

        let http_client = Client::builder()
            .user_agent("sentinai-agent/1.0")
            .build()
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let (tx, _rx) = broadcast::channel(100);

        Ok(Self {
            db,
            config: Arc::new(config),
            http_client,
            tx,
        })
    }
}
