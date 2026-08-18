//! Deadline handling for proposals awaiting votes.

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::vote_attention::{missing_approvers, vote_timeout};
use super::vote_tally::required_approvers_met;
use super::{AttentionItem, PersonaRuntimeState, Proposal, SwarmGovernance};

/// Whether the voting deadline has passed.
pub(super) fn past_deadline(proposal: &Proposal, now: DateTime<Utc>) -> bool {
    proposal
        .vote_deadline
        .is_some_and(|deadline| now > deadline)
}

/// Reject a proposal whose deadline passed without quorum or approvers.
///
/// Returns the attention item raised, or `None` when the proposal survives.
pub(super) fn reject_on_deadline(
    proposal: &mut Proposal,
    personas: &HashMap<String, PersonaRuntimeState>,
    governance: &SwarmGovernance,
    quorum_needed: usize,
    now: DateTime<Utc>,
) -> Option<AttentionItem> {
    if proposal.votes.len() < quorum_needed {
        let item = vote_timeout(proposal, now);
        proposal.status = super::ProposalStatus::Rejected;
        return Some(item);
    }
    if !required_approvers_met(proposal, personas, governance) {
        let item = missing_approvers(proposal, now);
        proposal.status = super::ProposalStatus::Rejected;
        return Some(item);
    }
    None
}
