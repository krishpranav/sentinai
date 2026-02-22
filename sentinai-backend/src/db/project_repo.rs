use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::project::Project;
use crate::error::AppError;

pub async fn create_project(
    pool: &PgPool,
    user_id: Uuid,
    name: &str,
    repository_url: &str,
) -> Result<Project, AppError> {
    let id = Uuid::new_v4();
    let project = sqlx::query_as::<_, Project>(
        r#"
        INSERT INTO projects (id, user_id, name, repository_url)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, name, repository_url, created_at
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(name)
    .bind(repository_url)
    .fetch_one(pool)
    .await?;

    Ok(project)
}

pub async fn list_projects(pool: &PgPool, user_id: Uuid) -> Result<Vec<Project>, AppError> {
    let projects = sqlx::query_as::<_, Project>(
        r#"
        SELECT id, user_id, name, repository_url, created_at
        FROM projects
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(projects)
}

pub async fn get_project(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<Option<Project>, AppError> {
    let project = sqlx::query_as::<_, Project>(
        r#"
        SELECT id, user_id, name, repository_url, created_at
        FROM projects
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(project_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(project)
}

pub async fn delete_project(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<u64, AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM projects
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(project_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::user_repo;
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
    async fn test_project_repository_crud() {
        let pool = get_pool().await;
        let github_id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;
        let user = user_repo::create_user(&pool, github_id, "test_proj_user", None)
            .await
            .unwrap();

        let proj = create_project(
            &pool,
            user.id,
            "test_project",
            "https://github.com/test/project",
        )
        .await
        .unwrap();
        assert_eq!(proj.name, "test_project");
        assert_eq!(proj.user_id, user.id);

        let found = get_project(&pool, proj.id, user.id)
            .await
            .unwrap()
            .expect("Project should exist");
        assert_eq!(found.id, proj.id);

        let list = list_projects(&pool, user.id).await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, proj.id);

        let rows_deleted = delete_project(&pool, proj.id, user.id).await.unwrap();
        assert_eq!(rows_deleted, 1);

        let not_found = get_project(&pool, proj.id, user.id).await.unwrap();
        assert!(not_found.is_none());

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user.id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
