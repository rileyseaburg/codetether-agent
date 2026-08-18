//! Resolution of one proposal against quorum, veto, and approver rules.

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::vote_deadline::{past_deadline, reject_on_deadline};
use super::vote_tally::{approved_by_majority, is_vetoed, required_approvers_met};
use super::{AttentionItem, PersonaRuntimeState, Proposal, ProposalStatus, SwarmGovernance};

/// Advance one `Created` proposal toward `Verified` or `Rejected`.
///
/// Returns any attention item raised while resolving.
pub(super) fn resolve(
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
