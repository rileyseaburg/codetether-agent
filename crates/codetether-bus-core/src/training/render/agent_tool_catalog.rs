//! AgentToolCatalog training record projection.
use super::*;
pub(super) fn build(
    tools: &Vec<crate::tool_catalog::ToolSchema>,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: Some(catalog::catalog_text(tools)),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
