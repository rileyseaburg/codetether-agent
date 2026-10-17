//! response artifact task operation.
use super::*;

pub fn response_artifact(message: &Message) -> Artifact {
    Artifact {
        artifact_id: Uuid::new_v4().to_string(),
        parts: message.parts.clone(),
        name: Some("response".to_string()),
        description: None,
        metadata: Default::default(),
        extensions: vec![],
    }
}
