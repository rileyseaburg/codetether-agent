//! Verified-proposal receipt and event construction.

use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::execution::{DecisionReceipt, ExecutionOutcome};
use crate::{Proposal, ThoughtEvent, ThoughtEventType};

/// Build the decision receipt recorded when a proposal executes.
///
/// # Arguments
/// * `proposal` - Proposal whose execution is recorded.
/// * `id` - Proposal record ID.
///
/// # Returns
/// The constructed receipt or event record.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::{*, execution::*, proposal::dispatch::ExecuteCtx};
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example(proposal: &Proposal, id: &str) {
/// let _result = codetether_cognition_core::proposal::dispatch::receipt_for(proposal, id);
/// # }
/// ```
pub fn receipt_for(proposal: &Proposal, id: &str) -> DecisionReceipt {
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
///
/// # Arguments
/// * `proposal` - Proposal whose execution is recorded.
/// * `receipt` - Receipt to reference in the emitted event.
/// * `id` - Proposal record ID.
///
/// # Returns
/// The constructed receipt or event record.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::{*, execution::*, proposal::dispatch::ExecuteCtx};
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example(proposal: &Proposal, receipt: &DecisionReceipt, id: &str) {
/// let _result = codetether_cognition_core::proposal::dispatch::executed_event(proposal, receipt, id);
/// # }
/// ```
pub fn executed_event(proposal: &Proposal, receipt: &DecisionReceipt, id: &str) -> ThoughtEvent {
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
