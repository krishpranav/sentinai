use axum::{
    extract::{Path, State},
    response::sse::{Event, Sse},
    routing::{get, post},
    Json, Router,
};
use std::convert::Infallible;
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};
use uuid::Uuid;

use crate::db::project_repo;
use crate::domain::project::{CreateProjectRequest, Project};
use crate::error::AppError;
use crate::routes::auth::AuthedUser;
use crate::state::AppState;

async fn create_project(
    State(state): State<AppState>,
    AuthedUser(user): AuthedUser,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<Project>, AppError> {
    let project =
        project_repo::create_project(&state.db, user.id, &payload.name, &payload.repository_url)
            .await?;

    Ok(Json(project))
}

async fn list_projects(
    State(state): State<AppState>,
    AuthedUser(user): AuthedUser,
) -> Result<Json<Vec<Project>>, AppError> {
    let projects = project_repo::list_projects(&state.db, user.id).await?;
    Ok(Json(projects))
}

async fn get_project(
    State(state): State<AppState>,
    AuthedUser(user): AuthedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Project>, AppError> {
    let project = project_repo::get_project(&state.db, id, user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".into()))?;

    Ok(Json(project))
}

async fn delete_project_handler(
    State(state): State<AppState>,
    AuthedUser(user): AuthedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let rows = project_repo::delete_project(&state.db, id, user.id).await?;
    if rows == 0 {
        return Err(AppError::NotFound("Project not found".into()));
    }

    Ok(Json(()))
}

async fn project_events(
    State(state): State<AppState>,
    AuthedUser(user): AuthedUser,
    Path(id): Path<Uuid>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let _ = project_repo::get_project(&state.db, id, user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".into()))?;

    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(move |item| {
        if let Ok(event) = item {
            let matches = match &event {
                crate::domain::realtime::RealtimeEvent::PipelineCreated(e) => e.project_id == id,
                crate::domain::realtime::RealtimeEvent::SecurityFindingCreated(e) => {
                    e.project_id == id
                }
            };
            if matches {
                let data = serde_json::to_string(&event).unwrap_or_default();
                Some(Ok(Event::default().data(data)))
            } else {
                None
            }
        } else {
            None
        }
    });

    Ok(Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new()))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects", post(create_project).get(list_projects))
        .route(
            "/projects/:id",
            get(get_project).delete(delete_project_handler),
        )
        .route("/projects/:id/events", get(project_events))
}
