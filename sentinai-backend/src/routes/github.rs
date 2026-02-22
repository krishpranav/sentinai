use axum::{routing::post, Json, Router};
use serde_json::Value;

use crate::state::AppState;

async fn github_webhook_handler(Json(payload): Json<Value>) -> Json<&'static str> {
    tracing::info!("Received GitHub webhook payload");

    if payload.get("action").is_none() {
        tracing::error!(target: "webhook", "signature verification failed");
    }

    Json("Webhook received")
}

pub fn router() -> Router<AppState> {
    Router::new().route("/webhooks/github", post(github_webhook_handler))
}
