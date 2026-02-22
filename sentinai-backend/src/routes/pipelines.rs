use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::db::project_repo;
use crate::domain::pipeline::Pipeline;
use crate::error::AppError;
use crate::routes::auth::AuthedUser;
use crate::services::pipeline_service;
use crate::state::AppState;

async fn generate_ci(
    State(state): State<AppState>,
    AuthedUser(user): AuthedUser,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Pipeline>, AppError> {
    let project = project_repo::get_project(&state.db, project_id, user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".into()))?;

    let pipeline =
        pipeline_service::generate_and_save_pipeline(&state, project.id, &project.repository_url)
            .await?;

    Ok(Json(pipeline))
}

async fn get_pipelines(
    State(state): State<AppState>,
    AuthedUser(user): AuthedUser,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<Pipeline>>, AppError> {
    let _ = project_repo::get_project(&state.db, project_id, user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".into()))?;

    let pipelines = pipeline_service::get_pipelines(&state, project_id).await?;
    Ok(Json(pipelines))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects/:id/generate-ci", post(generate_ci))
        .route("/projects/:id/pipelines", get(get_pipelines))
}
