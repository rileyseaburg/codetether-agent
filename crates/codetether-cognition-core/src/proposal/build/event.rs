//! Event announcing a newly created proposal.

use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{
    Proposal, ThoughtEvent, ThoughtEventType, ThoughtResult, ThoughtWorkItem, text as text_util,
};

/// Build the `proposal_created` event for `proposal`.
///
/// # Arguments
/// * `work` - Persona, phase and lineage associated with the completed work.
/// * `thought` - Completed thought and its provenance.
/// * `proposal` - Newly created proposal to announce.
///
/// # Returns
/// The constructed record with the existing field and provenance rules.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::proposal::build::created_event;
/// let work = ThoughtWorkItem { persona_id: "p1".into(), persona_name: "Reviewer".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, thought_count: 1, phase: ThoughtPhase::Reflect };
/// let thought = ThoughtResult { source: "example", model: None, finish_reason: None, thinking: "Review the change".into(), prompt_tokens: None, completion_tokens: None, total_tokens: None, latency_ms: 0, error: None };
/// let proposal = codetether_cognition_core::proposal::build::build_proposal(&work, &thought, &SwarmGovernance::default(), 2);
/// let event = created_event(&work, &thought, &proposal);
/// assert_eq!(event.payload["proposal_id"], proposal.id);
/// ```
pub fn created_event(
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
