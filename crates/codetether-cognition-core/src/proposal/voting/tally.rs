//! Vote tallying predicates for proposal resolution.

use std::collections::HashMap;

use crate::{PersonaRuntimeState, Proposal, ProposalVote, SwarmGovernance};

/// Whether a persona holding a veto role voted to veto.
///
/// # Arguments
/// * `proposal` - Proposal being evaluated or updated.
/// * `personas` - Persona records indexed by ID.
/// * `governance` - Voting policy for the proposal.
///
/// # Returns
/// The result of the policy described above.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::proposal::voting::is_vetoed;
/// let mut proposal = Proposal { id: "q1".into(), persona_id: "p1".into(), title: "Review".into(), rationale: "Verify".into(), evidence_refs: vec![], risk: ProposalRisk::Low, status: ProposalStatus::Created, created_at: chrono::Utc::now(), votes: Default::default(), vote_deadline: None, votes_requested: false, quorum_needed: 1 };
/// let personas = std::collections::HashMap::new();
/// let governance = SwarmGovernance::default();
/// let result = is_vetoed(&proposal, &personas, &governance);
/// assert!(!result);
/// ```
pub fn is_vetoed(
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
///
/// # Arguments
/// * `proposal` - Proposal being evaluated or updated.
/// * `personas` - Persona records indexed by ID.
/// * `governance` - Voting policy for the proposal.
///
/// # Returns
/// The result of the policy described above.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::proposal::voting::required_approvers_met;
/// let mut proposal = Proposal { id: "q1".into(), persona_id: "p1".into(), title: "Review".into(), rationale: "Verify".into(), evidence_refs: vec![], risk: ProposalRisk::Low, status: ProposalStatus::Created, created_at: chrono::Utc::now(), votes: Default::default(), vote_deadline: None, votes_requested: false, quorum_needed: 1 };
/// let personas = std::collections::HashMap::new();
/// let governance = SwarmGovernance::default();
/// let result = required_approvers_met(&proposal, &personas, &governance);
/// assert!(result);
/// ```
pub fn required_approvers_met(
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
/// use codetether_cognition_core::proposal::voting::approved_by_majority;
/// let mut proposal = Proposal { id: "q1".into(), persona_id: "p1".into(), title: "Review".into(), rationale: "Verify".into(), evidence_refs: vec![], risk: ProposalRisk::Low, status: ProposalStatus::Created, created_at: chrono::Utc::now(), votes: Default::default(), vote_deadline: None, votes_requested: false, quorum_needed: 1 };
/// let result = approved_by_majority(&proposal);
/// assert!(!result);
/// ```
pub fn approved_by_majority(proposal: &Proposal) -> bool {
    let count = |target: ProposalVote| proposal.votes.values().filter(|v| **v == target).count();
    count(ProposalVote::Approve) > count(ProposalVote::Reject)
}
