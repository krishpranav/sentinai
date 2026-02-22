use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::security::SecurityFinding;
use crate::error::AppError;

pub async fn create_finding(
    pool: &PgPool,
    project_id: Uuid,
    severity: &str,
    description: &str,
) -> Result<SecurityFinding, AppError> {
    let id = Uuid::new_v4();
    let finding = sqlx::query_as::<_, SecurityFinding>(
        r#"
        INSERT INTO security_findings (id, project_id, severity, description)
        VALUES ($1, $2, $3, $4)
        RETURNING id, project_id, severity, description, resolved, created_at
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(severity)
    .bind(description)
    .fetch_one(pool)
    .await?;

    Ok(finding)
}

pub async fn get_findings_for_project(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<SecurityFinding>, AppError> {
    let findings = sqlx::query_as::<_, SecurityFinding>(
        r#"
        SELECT id, project_id, severity, description, resolved, created_at
        FROM security_findings
        WHERE project_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    Ok(findings)
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
    #[ignore]
    async fn test_security_repository_crud() {
        let pool = get_pool().await;
        let github_id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;
        let user = user_repo::create_user(&pool, github_id, "test_sec_user", None)
            .await
            .unwrap();
        let proj = project_repo::create_project(&pool, user.id, "test_sec_proj", "https://url")
            .await
            .unwrap();

        let finding = create_finding(&pool, proj.id, "high", "Found exposed token")
            .await
            .unwrap();
        assert_eq!(finding.severity, "high");
        assert_eq!(finding.project_id, proj.id);
        assert_eq!(finding.resolved, false);

        let findings = get_findings_for_project(&pool, proj.id).await.unwrap();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].id, finding.id);

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user.id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
