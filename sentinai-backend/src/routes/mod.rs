use crate::state::AppState;
use axum::Router;

pub mod auth;
pub mod github;
pub mod health;
pub mod pipelines;
pub mod projects;
pub mod security;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(health::router())
        .merge(auth::router())
        .merge(projects::router())
        .merge(pipelines::router())
        .merge(security::router())
        .merge(github::router())
}
