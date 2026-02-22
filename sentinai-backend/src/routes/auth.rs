use crate::db::user_repo;
use crate::domain::user::User;
use crate::error::AppError;
use crate::services::github_service;
use crate::state::AppState;
use crate::utils::jwt;
use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::request::Parts,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub struct AuthedUser(pub User);

#[async_trait]
impl FromRequestParts<AppState> for AuthedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        let token = match auth_header {
            Some(t) => t,
            None => return Err(AppError::AuthError("Missing authorization header".into())),
        };

        let claims = jwt::verify_token(token, &state.config.jwt_secret)?;

        let user = user_repo::find_by_id(&state.db, claims.sub)
            .await?
            .ok_or_else(|| AppError::AuthError("User not found".into()))?;

        Ok(AuthedUser(user))
    }
}

#[derive(Deserialize)]
pub struct AuthGithubRequest {
    pub access_token: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

async fn auth_github(
    State(state): State<AppState>,
    Json(payload): Json<AuthGithubRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let gh_user = github_service::get_user_profile(&state, &payload.access_token).await?;

    let user = match user_repo::find_by_github_id(&state.db, gh_user.id).await? {
        Some(u) => u,
        None => {
            user_repo::create_user(
                &state.db,
                gh_user.id,
                &gh_user.login,
                gh_user.email.as_deref(),
            )
            .await?
        }
    };

    let token = jwt::create_token(user.id, &state.config.jwt_secret)?;

    Ok(Json(AuthResponse { token, user }))
}

async fn get_me(AuthedUser(user): AuthedUser) -> Json<User> {
    Json(user)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/github", post(auth_github))
        .route("/auth/me", get(get_me))
}
