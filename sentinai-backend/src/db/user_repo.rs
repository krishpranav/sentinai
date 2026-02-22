use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::user::User;
use crate::error::AppError;

pub async fn find_by_github_id(pool: &PgPool, github_id: i64) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, github_id, username, email, created_at
        FROM users
        WHERE github_id = $1
        "#,
    )
    .bind(github_id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, github_id, username, email, created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn create_user(
    pool: &PgPool,
    github_id: i64,
    username: &str,
    email: Option<&str>,
) -> Result<User, AppError> {
    let id = Uuid::new_v4();
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, github_id, username, email)
        VALUES ($1, $2, $3, $4)
        RETURNING id, github_id, username, email, created_at
        "#,
    )
    .bind(id)
    .bind(github_id)
    .bind(username)
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
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
    async fn test_user_repository_crud() {
        let pool = get_pool().await;
        let github_id: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;

        let user = create_user(&pool, github_id, "test_repo_user", Some("repo@test.com"))
            .await
            .unwrap();
        assert_eq!(user.github_id, github_id);
        assert_eq!(user.username, "test_repo_user");

        let found = find_by_github_id(&pool, github_id)
            .await
            .unwrap()
            .expect("User should exist");
        assert_eq!(found.id, user.id);

        let found_id = find_by_id(&pool, user.id)
            .await
            .unwrap()
            .expect("User should exist");
        assert_eq!(found_id.github_id, github_id);

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user.id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
