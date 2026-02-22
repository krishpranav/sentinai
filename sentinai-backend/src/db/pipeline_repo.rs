use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::pipeline::Pipeline;
use crate::error::AppError;

pub async fn create_pipeline(
    pool: &PgPool,
    project_id: Uuid,
    yaml_config: &str,
) -> Result<Pipeline, AppError> {
    let id = Uuid::new_v4();
    let pipeline = sqlx::query_as::<_, Pipeline>(
        r#"
        INSERT INTO pipelines (id, project_id, yaml_config)
        VALUES ($1, $2, $3)
        RETURNING id, project_id, yaml_config, created_at
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(yaml_config)
    .fetch_one(pool)
    .await?;

    Ok(pipeline)
}

pub async fn get_pipelines_for_project(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<Pipeline>, AppError> {
    let pipelines = sqlx::query_as::<_, Pipeline>(
        r#"
        SELECT id, project_id, yaml_config, created_at
        FROM pipelines
        WHERE project_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    Ok(pipelines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{project_repo, user_repo};
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    async fn get_pool() -> PgPool {
        dotenvy::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPoolOptions::new()
            .max_connections(2)
            .connect(&db_url)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_pipeline_repository_crud() {
        let pool = get_pool().await;
        let github_id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;
        let user = user_repo::create_user(&pool, github_id, "test_pipe_user", None)
            .await
            .unwrap();
        let proj = project_repo::create_project(&pool, user.id, "test_pipe_proj", "https://url")
            .await
            .unwrap();

        let yaml_content = "name: Test Pipeline";
        let pipeline = create_pipeline(&pool, proj.id, yaml_content).await.unwrap();
        assert_eq!(pipeline.yaml_config, yaml_content);
        assert_eq!(pipeline.project_id, proj.id);

        let pipelines = get_pipelines_for_project(&pool, proj.id).await.unwrap();
        assert_eq!(pipelines.len(), 1);
        assert_eq!(pipelines[0].id, pipeline.id);

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user.id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
