//! Verified-proposal receipt and event construction.

use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use super::executor::{DecisionReceipt, ExecutionOutcome};
use super::{Proposal, ThoughtEvent, ThoughtEventType};

/// Build the decision receipt recorded when a proposal executes.
pub(super) fn receipt_for(proposal: &Proposal, id: &str) -> DecisionReceipt {
    DecisionReceipt {
        id: Uuid::new_v4().to_string(),
        proposal_id: id.to_string(),
        inputs: proposal.evidence_refs.clone(),
        governance_decision: format!("Approved with {} votes", proposal.votes.len()),
        capability_leases: Vec::new(),
        tool_invocations: Vec::new(),
        outcome: ExecutionOutcome::Success {
            summary: format!("Proposal '{}' executed", proposal.title),
        },
        created_at: Utc::now(),
    }
}

/// Event announcing that a proposal executed.
pub(super) fn executed_event(
    proposal: &Proposal,
    receipt: &DecisionReceipt,
    id: &str,
) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::ActionExecuted,
        persona_id: Some(proposal.persona_id.clone()),
        swarm_id: None,
        timestamp: Utc::now(),
        payload: json!({
            "receipt_id": receipt.id,
            "proposal_id": id,
            "outcome": "success",
            "summary": format!("Proposal '{}' executed", proposal.title),
        }),
    }
}
