//! Provider resolution and model draft invocation for pending objectives.
use super::DraftHost;
use crate::PlannedOkrDraft;
use std::sync::Arc;
pub(super) async fn plan<H: DraftHost>(
    task: &str,
    model_ref: &str,
    agent_count: usize,
    registry: &Arc<H::Registry>,
) -> Option<PlannedOkrDraft> {
    let (provider, model_name) = H::resolve(registry, model_ref)?;
    crate::draft(task, model_name, agent_count, &provider).await
}
