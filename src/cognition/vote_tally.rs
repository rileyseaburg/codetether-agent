//! Vote tallying predicates for proposal resolution.

use std::collections::HashMap;

use super::{PersonaRuntimeState, Proposal, ProposalVote, SwarmGovernance};

/// Whether a persona holding a veto role voted to veto.
pub(super) fn is_vetoed(
    proposal: &Proposal,
    personas: &HashMap<String, PersonaRuntimeState>,
    governance: &SwarmGovernance,
) -> bool {
    proposal.votes.iter().any(|(voter_id, vote)| {
        *vote == ProposalVote::Veto
            && personas
                .get(voter_id)
                .is_some_and(|voter| governance.veto_roles.contains(&voter.identity.role))
    })
}

/// Whether every role required for this risk level has approved.
pub(super) fn required_approvers_met(
    proposal: &Proposal,
    personas: &HashMap<String, PersonaRuntimeState>,
    governance: &SwarmGovernance,
) -> bool {
    let required = governance
        .required_approvers_by_role
        .get(&proposal.risk)
        .cloned()
        .unwrap_or_default();
    required.iter().all(|role| {
        proposal.votes.iter().any(|(voter_id, vote)| {
            *vote == ProposalVote::Approve
                && personas
                    .get(voter_id)
                    .is_some_and(|p| &p.identity.role == role)
        })
    })
}

/// Whether approvals strictly outnumber rejections.
pub(super) fn approved_by_majority(proposal: &Proposal) -> bool {
    let count = |target: ProposalVote| proposal.votes.values().filter(|v| **v == target).count();
    count(ProposalVote::Approve) > count(ProposalVote::Reject)
}
