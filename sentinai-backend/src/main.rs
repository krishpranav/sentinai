mod app;
mod config;
mod db;
mod domain;
mod error;
mod routes;
mod services;
mod state;
mod utils;

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::logging::init_tracing();
    dotenvy::dotenv().ok();

    let config = config::Config::from_env()?;
    let app = app::create_app(config.clone()).await?;

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
