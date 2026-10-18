//! Pending objective creation with existing model-draft fallbacks.
use super::{DraftHost, draft};
use crate::{PendingOkrApproval, default_relay_okr_template, okr_from_planned_draft};
/// Create a pending approval objective, falling back when drafting is unavailable.
/// # Arguments
/// * `task` - Requested relay task.
/// * `agent_count` - Number of agents included in the draft.
/// * `model` - Selected model reference.
/// # Returns
/// Pending approval with a model draft or the existing fallback template.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_relay_okr::interactive::DraftHost>() {
/// let pending = codetether_relay_okr::interactive::propose::<H>("task".into(), 3, "model".into()).await;
/// # }
/// ```
pub async fn propose<H: DraftHost>(
    task: String,
    agent_count: usize,
    model: String,
) -> PendingOkrApproval {
    let mut pending = PendingOkrApproval::new(task, agent_count, model);
    let okr_id = pending.okr.id;
    let registry = H::load_registry().await;
    let task = pending.task.clone();
    let agent_count = pending.agent_count;
    let model = pending.model.clone();
    let (okr, draft_note) = if let Some(registry) = &registry {
        match draft::plan::<H>(&task, &model, agent_count, registry).await {
            Some(planned) => (okr_from_planned_draft(okr_id, &task, planned), None),
            None => (
                default_relay_okr_template(okr_id, &task),
                Some("(OKR: fallback template — model draft parse failed)".to_string()),
            ),
        }
    } else {
        (
            default_relay_okr_template(okr_id, &task),
            Some("(OKR: fallback template — provider unavailable)".to_string()),
        )
    };
    pending.okr = okr;
    pending.draft_note = draft_note;
    pending
}
