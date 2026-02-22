use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::db::project_repo;
use crate::domain::security::SecurityFinding;
use crate::error::AppError;
use crate::routes::auth::AuthedUser;
use crate::services::security_service;
use crate::state::AppState;

async fn run_scan(
    State(state): State<AppState>,
    AuthedUser(user): AuthedUser,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<SecurityFinding>>, AppError> {
    let _ = project_repo::get_project(&state.db, project_id, user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".into()))?;

    let findings = security_service::run_security_scan(&state, project_id).await?;

    Ok(Json(findings))
}

async fn get_findings(
    State(state): State<AppState>,
    AuthedUser(user): AuthedUser,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<SecurityFinding>>, AppError> {
    let _ = project_repo::get_project(&state.db, project_id, user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".into()))?;

    let findings = security_service::get_security_findings(&state, project_id).await?;

    Ok(Json(findings))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects/:id/security", get(get_findings))
        .route("/projects/:id/security/scan", post(run_scan))
}
