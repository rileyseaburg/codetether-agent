use crate::a2a::types::{Artifact, TaskState};
use crate::bus::{BusEnvelope, BusMessage};
use axum::response::{IntoResponse, Sse};
use std::time::Duration;

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

pub(super) async fn render(messages: Vec<BusEnvelope>, capacity: usize) -> String {
    let (tx, rx) = tokio::sync::broadcast::channel(capacity);
    for message in messages {
        tx.send(message).unwrap();
    }
    drop(tx);
    let body = Sse::new(super::super::service::events(rx, "abc".into()))
        .into_response()
        .into_body();
    let bytes = tokio::time::timeout(Duration::from_secs(2), axum::body::to_bytes(body, 8192))
        .await
        .expect("closed bus must finish SSE")
        .unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}
