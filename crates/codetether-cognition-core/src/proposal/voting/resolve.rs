//! Resolution of one proposal against quorum, veto, and approver rules.

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::deadline::{past_deadline, reject_on_deadline};
use super::tally::{approved_by_majority, is_vetoed, required_approvers_met};
use crate::{AttentionItem, PersonaRuntimeState, Proposal, ProposalStatus, SwarmGovernance};

/// Advance one `Created` proposal toward `Verified` or `Rejected`.
///
/// Returns any attention item raised while resolving.
///
/// # Arguments
/// * `proposal` - Proposal being evaluated or updated.
/// * `personas` - Persona records indexed by ID.
/// * `governance` - Voting policy for the proposal.
/// * `now` - Time used for deadline and ranking decisions.
///
/// # Returns
/// The result of the policy described above.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::proposal::voting::resolve;
/// let mut proposal = Proposal { id: "q1".into(), persona_id: "p1".into(), title: "Review".into(), rationale: "Verify".into(), evidence_refs: vec![], risk: ProposalRisk::Low, status: ProposalStatus::Created, created_at: chrono::Utc::now(), votes: Default::default(), vote_deadline: None, votes_requested: false, quorum_needed: 1 };
/// let personas = std::collections::HashMap::new();
/// let governance = SwarmGovernance::default();
/// let now = chrono::Utc::now();
/// let result = resolve(&mut proposal, &personas, &governance, now);
/// assert!(result.is_none());
/// ```
pub fn resolve(
    proposal: &mut Proposal,
    personas: &HashMap<String, PersonaRuntimeState>,
    governance: &SwarmGovernance,
    now: DateTime<Utc>,
) -> Option<AttentionItem> {
    let quorum_needed = proposal.quorum_needed.max(1);
    if past_deadline(proposal, now)
        && let Some(item) = reject_on_deadline(proposal, personas, governance, quorum_needed, now)
    {
        return Some(item);
    }
    if proposal.votes.len() < quorum_needed {
        return None;
    }
    if is_vetoed(proposal, personas, governance) {
        proposal.status = ProposalStatus::Rejected;
        return None;
    }
    if !required_approvers_met(proposal, personas, governance) {
        // Wait for the required approvers rather than deciding early.
        return None;
    }
    proposal.status = if approved_by_majority(proposal) {
        ProposalStatus::Verified
    } else {
        ProposalStatus::Rejected
    };
    None
}
