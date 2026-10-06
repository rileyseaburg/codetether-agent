pub(super) use super::transport::render;
use crate::a2a::types::{Artifact, TaskState};
use crate::bus::{BusEnvelope, BusMessage};

pub(super) fn envelope(topic: &str, message: BusMessage) -> BusEnvelope {
    BusEnvelope {
        id: "test-envelope".into(),
        topic: topic.into(),
        sender_id: "test-worker".into(),
        correlation_id: None,
        timestamp: chrono::Utc::now(),
        message,
    }
}

pub(super) fn update(topic: &str, id: &str) -> BusEnvelope {
    envelope(
        topic,
        BusMessage::TaskUpdate {
            task_id: id.into(),
            state: TaskState::Working,
            message: Some("chunk".into()),
        },
    )
}

pub(super) fn artifact(topic: &str, id: &str) -> BusEnvelope {
    envelope(
        topic,
        BusMessage::ArtifactUpdate {
            task_id: id.into(),
            artifact: Artifact {
                artifact_id: "artifact-1".into(),
                parts: vec![],
                name: None,
                description: None,
                metadata: Default::default(),
                extensions: vec![],
            },
        },
    )
}
