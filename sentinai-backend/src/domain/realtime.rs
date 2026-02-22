use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum RealtimeEvent {
    PipelineCreated(PipelineEvent),
    SecurityFindingCreated(SecurityEvent),
}

#[derive(Clone, Serialize, Debug)]
pub struct PipelineEvent {
    pub project_id: Uuid,
    pub pipeline_id: Uuid,
}

#[derive(Clone, Serialize, Debug)]
pub struct SecurityEvent {
    pub project_id: Uuid,
    pub finding_id: Uuid,
    pub severity: String,
    pub description: String,
}
