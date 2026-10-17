//! ArtifactUpdate training record projection.
use super::*;
pub(super) fn build(
    task_id: &String,
    artifact: &Artifact,
    meta: TrainingMetadata,
) -> TrainingRecord {
    {
        let artifact_text = parts_to_text(&artifact.parts);
        TrainingRecord {
            role: "system".into(),
            content: Some(format!(
                "Task `{task_id}` artifact `{}`: {artifact_text}",
                artifact.artifact_id
            )),
            tool_calls: None,
            tool_call_id: None,
            name: None,
            metadata: meta,
        }
    }
}
