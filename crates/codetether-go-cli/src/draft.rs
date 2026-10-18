//! Model-authored objective with the existing CLI fallback explanations.
use codetether_autochat_core::model_rotation::ModelRegistry;
use codetether_okr::Okr;
use codetether_relay_plan::{default_relay_okr_template, okr_from_planned_draft};
use codetether_relay_planner::plan_okr_draft_with_registry;
use std::sync::Arc;
use uuid::Uuid;
pub(super) async fn prepare<R: ModelRegistry>(
    task: &str,
    model: &str,
    agent_count: usize,
    okr_id: Uuid,
    registry_for_draft: &Option<Arc<R>>,
) -> (Okr, Option<String>) {
    let (okr, draft_note) = if let Some(registry) = registry_for_draft {
        match plan_okr_draft_with_registry(task, model, agent_count, registry).await {
            Some(planned) => (okr_from_planned_draft(okr_id, task, planned), None),
            None => (
                default_relay_okr_template(okr_id, task),
                Some("(OKR: fallback template — model draft parse failed)".to_string()),
            ),
        }
    } else {
        (
            default_relay_okr_template(okr_id, task),
            Some("(OKR: fallback template — provider unavailable)".to_string()),
        )
    };

    (okr, draft_note)
}
