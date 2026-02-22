use uuid::Uuid;

use crate::db::security_repo;
use crate::domain::realtime::{RealtimeEvent, SecurityEvent};
use crate::domain::security::SecurityFinding;
use crate::error::AppError;
use crate::state::AppState;

#[tracing::instrument(name = "security", skip(state), fields(project_id = %project_id))]
pub async fn run_security_scan(
    state: &AppState,
    project_id: Uuid,
) -> Result<Vec<SecurityFinding>, AppError> {
    let mock_findings = vec![
        ("high", "Detected outdated OpenSSL dependency in Cargo.lock"),
        ("medium", "Missing security headers in API response"),
    ];

    let mut results = Vec::new();

    for (severity, desc) in mock_findings {
        tracing::warn!(
            severity = severity,
            finding = desc,
            "1 vulnerable dependency found"
        );
        let finding = security_repo::create_finding(&state.db, project_id, severity, desc).await?;

        let _ = state
            .tx
            .send(RealtimeEvent::SecurityFindingCreated(SecurityEvent {
                project_id,
                finding_id: finding.id,
                severity: finding.severity.clone(),
                description: finding.description.clone(),
            }));

        results.push(finding);
    }

    Ok(results)
}

#[tracing::instrument(name = "security", skip(state), fields(project_id = %project_id))]
pub async fn get_security_findings(
    state: &AppState,
    project_id: Uuid,
) -> Result<Vec<SecurityFinding>, AppError> {
    security_repo::get_findings_for_project(&state.db, project_id).await
}
