use axum::http::Request;
use axum::response::Response;
use axum::{middleware, Router};
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};

use crate::config::Config;
use crate::error::AppError;
use crate::routes;
use crate::state::AppState;

pub async fn create_app(config: Config) -> Result<Router, AppError> {
    let state = AppState::new(config).await?;

    let app = Router::new()
        .merge(routes::router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let req_id = request
                        .extensions()
                        .get::<crate::utils::middleware::RequestId>()
                        .map(|r| r.0.as_str())
                        .unwrap_or("unknown");
                    info_span!(
                        "request",
                        method = %request.method(),
                        path = %request.uri().path(),
                        id = %req_id,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    tracing::debug!("Started processing request");
                })
                .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                    let status = response.status().as_u16();
                    if status >= 500 {
                        tracing::error!(status = status, latency = ?latency, "Request failed");
                    } else if status >= 400 {
                        tracing::warn!(status = status, latency = ?latency, "Client error");
                    } else {
                        tracing::info!(status = status, latency = ?latency, "Request completed");
                    }
                }),
        )
        .layer(middleware::from_fn(
            crate::utils::middleware::request_id_middleware,
        ))
        .layer(CorsLayer::permissive())
        .with_state(state);

    Ok(app)
}
