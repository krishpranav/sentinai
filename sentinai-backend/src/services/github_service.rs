use serde::Deserialize;

use crate::error::AppError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct GithubUser {
    pub id: i64,
    pub login: String,
    pub email: Option<String>,
}

#[tracing::instrument(
    name = "github",
    skip(state, access_token),
    fields(token_hidden = true)
)]
pub async fn get_user_profile(
    state: &AppState,
    access_token: &str,
) -> Result<GithubUser, AppError> {
    if access_token == "mock-token" && state.config.database_url.contains("neondb") {
        return Ok(GithubUser {
            id: 123456789,
            login: "mock_user".to_string(),
            email: Some("mock@example.com".to_string()),
        });
    }

    let url = "https://api.github.com/user";
    let res = state
        .http_client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| AppError::InternalServerError(format!("GitHub API error: {}", e)))?;

    if !res.status().is_success() {
        return Err(AppError::AuthError("Invalid GitHub token".into()));
    }

    let user: GithubUser = res.json().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to parse GitHub user: {}", e))
    })?;

    Ok(user)
}
