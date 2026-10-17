//! Deadline handling for proposals awaiting votes.

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::attention::{missing_approvers, vote_timeout};
use super::tally::required_approvers_met;
use crate::{AttentionItem, PersonaRuntimeState, Proposal, SwarmGovernance};

/// Whether the voting deadline has passed.
///
/// # Arguments
/// * `proposal` - Proposal being evaluated or updated.
///
/// # Returns
/// The result of the policy described above.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::proposal::voting::past_deadline;
/// let mut proposal = Proposal { id: "q1".into(), persona_id: "p1".into(), title: "Review".into(), rationale: "Verify".into(), evidence_refs: vec![], risk: ProposalRisk::Low, status: ProposalStatus::Created, created_at: chrono::Utc::now(), votes: Default::default(), vote_deadline: None, votes_requested: false, quorum_needed: 1 };
/// let now = chrono::Utc::now();
/// let result = past_deadline(&proposal, now);
/// assert!(!result);
/// ```
pub fn past_deadline(proposal: &Proposal, now: DateTime<Utc>) -> bool {
    proposal
        .vote_deadline
        .is_some_and(|deadline| now > deadline)
}

/// Reject a proposal whose deadline passed without quorum or approvers.
///
/// Returns the attention item raised, or `None` when the proposal survives.
///
/// # Arguments
/// * `proposal` - Proposal being evaluated or updated.
/// * `personas` - Persona records indexed by ID.
/// * `governance` - Voting policy for the proposal.
/// * `quorum_needed` - Required total number of votes.
/// * `now` - Time used for deadline and ranking decisions.
///
/// # Returns
/// The result of the policy described above.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::proposal::voting::reject_on_deadline;
/// let mut proposal = Proposal { id: "q1".into(), persona_id: "p1".into(), title: "Review".into(), rationale: "Verify".into(), evidence_refs: vec![], risk: ProposalRisk::Low, status: ProposalStatus::Created, created_at: chrono::Utc::now(), votes: Default::default(), vote_deadline: None, votes_requested: false, quorum_needed: 1 };
/// let personas = std::collections::HashMap::new();
/// let governance = SwarmGovernance::default();
/// let now = chrono::Utc::now();
/// let result = reject_on_deadline(&mut proposal, &personas, &governance, 1, now);
/// assert!(result.is_some());
/// ```
pub fn reject_on_deadline(
    proposal: &mut Proposal,
    personas: &HashMap<String, PersonaRuntimeState>,
    governance: &SwarmGovernance,
    quorum_needed: usize,
    now: DateTime<Utc>,
) -> Option<AttentionItem> {
    if proposal.votes.len() < quorum_needed {
        let item = vote_timeout(proposal, now);
        proposal.status = crate::ProposalStatus::Rejected;
        return Some(item);
    }
    if !required_approvers_met(proposal, personas, governance) {
        let item = missing_approvers(proposal, now);
        proposal.status = crate::ProposalStatus::Rejected;
        return Some(item);
    }
    None
}
