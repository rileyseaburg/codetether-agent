//! Event announcing a newly created proposal.

use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use super::{Proposal, ThoughtEvent, ThoughtEventType, ThoughtResult, ThoughtWorkItem, text_util};

/// Build the `proposal_created` event for `proposal`.
pub(super) fn created_event(
    work: &ThoughtWorkItem,
    thought: &ThoughtResult,
    proposal: &Proposal,
) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::ProposalCreated,
        persona_id: Some(work.persona_id.clone()),
        swarm_id: work.swarm_id.clone(),
        timestamp: Utc::now(),
        payload: json!({
            "proposal_id": proposal.id,
            "title": proposal.title,
            "rationale_excerpt": text_util::trim_for_storage(&proposal.rationale, 220),
            "source": thought.source,
            "model": thought.model,
        }),
    }
}
