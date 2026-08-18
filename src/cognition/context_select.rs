//! Selection of recent events relevant to one persona.

use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{ThoughtEvent, ThoughtEventType};

/// Swarm-wide event types every persona should see, even when unattributed.
fn is_shared(event_type: ThoughtEventType) -> bool {
    matches!(
        event_type,
        ThoughtEventType::CheckResult
            | ThoughtEventType::ProposalCreated
            | ThoughtEventType::SnapshotCompressed
            | ThoughtEventType::WorkspaceUpdated
            | ThoughtEventType::ActionExecuted
            | ThoughtEventType::BudgetPaused
    )
}

/// Return up to `limit` recent events for `persona_id`, oldest first.
pub(super) async fn recent_persona_context(
    events: &Arc<RwLock<VecDeque<ThoughtEvent>>>,
    persona_id: &str,
    limit: usize,
) -> Vec<ThoughtEvent> {
    let lock = events.read().await;
    let mut selected: Vec<ThoughtEvent> = lock
        .iter()
        .rev()
        .filter(|event| {
            event.persona_id.as_deref() == Some(persona_id)
                || (event.persona_id.is_none() && is_shared(event.event_type))
        })
        .take(limit)
        .cloned()
        .collect();
    selected.reverse();
    selected
}
