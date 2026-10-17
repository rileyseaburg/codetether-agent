//! Build the stream response artifact.
use super::*;
pub(super) fn artifact(result_text: &String) -> Artifact {
    Artifact {
        artifact_id: Uuid::new_v4().to_string(),
        parts: vec![Part::Text {
            text: result_text.clone(),
        }],
        name: Some("response".to_string()),
        description: None,
        metadata: std::collections::HashMap::new(),
        extensions: vec![],
    }
}
