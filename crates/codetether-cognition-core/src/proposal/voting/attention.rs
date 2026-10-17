//! Attention items raised when proposal voting stalls.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{AttentionItem, AttentionSource, Proposal};

/// Attention for a proposal whose deadline passed without quorum.
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
/// use codetether_cognition_core::proposal::voting::vote_timeout;
/// let mut proposal = Proposal { id: "q1".into(), persona_id: "p1".into(), title: "Review".into(), rationale: "Verify".into(), evidence_refs: vec![], risk: ProposalRisk::Low, status: ProposalStatus::Created, created_at: chrono::Utc::now(), votes: Default::default(), vote_deadline: None, votes_requested: false, quorum_needed: 1 };
/// let now = chrono::Utc::now();
/// let result = vote_timeout(&proposal, now);
/// assert_eq!(result.source_type, AttentionSource::ProposalTimeout);
/// ```
pub fn vote_timeout(proposal: &Proposal, now: DateTime<Utc>) -> AttentionItem {
    item(
        format!("Proposal vote timeout: {}", proposal.title),
        0.6,
        proposal,
        now,
    )
}

/// Attention for a proposal that reached quorum without required approvers.
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
/// use codetether_cognition_core::proposal::voting::missing_approvers;
/// let mut proposal = Proposal { id: "q1".into(), persona_id: "p1".into(), title: "Review".into(), rationale: "Verify".into(), evidence_refs: vec![], risk: ProposalRisk::Low, status: ProposalStatus::Created, created_at: chrono::Utc::now(), votes: Default::default(), vote_deadline: None, votes_requested: false, quorum_needed: 1 };
/// let now = chrono::Utc::now();
/// let result = missing_approvers(&proposal, now);
/// assert_eq!(result.source_type, AttentionSource::ProposalTimeout);
/// ```
pub fn missing_approvers(proposal: &Proposal, now: DateTime<Utc>) -> AttentionItem {
    item(
        format!("Missing required approvers: {}", proposal.title),
        0.7,
        proposal,
        now,
    )
}

fn item(topic: String, priority: f32, proposal: &Proposal, now: DateTime<Utc>) -> AttentionItem {
    AttentionItem {
        id: Uuid::new_v4().to_string(),
        topic,
        topic_tags: Vec::new(),
        priority,
        source_type: AttentionSource::ProposalTimeout,
        source_id: proposal.id.clone(),
        assigned_persona: None,
        created_at: now,
        resolved_at: None,
    }
}
