use uuid::Uuid;

use crate::db::pipeline_repo;
use crate::domain::pipeline::Pipeline;
use crate::domain::realtime::{PipelineEvent, RealtimeEvent};
use crate::error::AppError;
use crate::services::ci_generator::{detect_project_type, generate_ci_yaml};
use crate::state::AppState;

#[tracing::instrument(name = "pipeline_service", skip(state), fields(project_id = %project_id))]
pub async fn generate_and_save_pipeline(
    state: &AppState,
    project_id: Uuid,
    repository_url: &str,
) -> Result<Pipeline, AppError> {
    let project_type = detect_project_type(repository_url);

    let yaml = generate_ci_yaml(project_type);

    let pipeline = pipeline_repo::create_pipeline(&state.db, project_id, &yaml).await?;

    let _ = state.tx.send(RealtimeEvent::PipelineCreated(PipelineEvent {
        project_id,
        pipeline_id: pipeline.id,
    }));

    Ok(pipeline)
}

#[tracing::instrument(name = "pipeline_service", skip(state), fields(project_id = %project_id))]
pub async fn get_pipelines(state: &AppState, project_id: Uuid) -> Result<Vec<Pipeline>, AppError> {
    pipeline_repo::get_pipelines_for_project(&state.db, project_id).await
}
