use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SecurityFinding {
    pub id: Uuid,
    pub project_id: Uuid,
    pub severity: String,
    pub description: String,
    pub resolved: bool,
    pub created_at: DateTime<Utc>,
}
