//! Attention items raised when proposal voting stalls.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{AttentionItem, AttentionSource, Proposal};

/// Attention for a proposal whose deadline passed without quorum.
pub(super) fn vote_timeout(proposal: &Proposal, now: DateTime<Utc>) -> AttentionItem {
    item(
        format!("Proposal vote timeout: {}", proposal.title),
        0.6,
        proposal,
        now,
    )
}

/// Attention for a proposal that reached quorum without required approvers.
pub(super) fn missing_approvers(proposal: &Proposal, now: DateTime<Utc>) -> AttentionItem {
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
