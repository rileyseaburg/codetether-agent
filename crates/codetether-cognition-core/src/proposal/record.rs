//! Proposal contract: think first, execute through governance gates.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ProposalRisk, ProposalStatus, ProposalVote};

/// A proposed action awaiting verification, quorum, and execution.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = Proposal { id: "q1".into(), persona_id: "p1".into(), title: "Review".into(), rationale: "Verify changes".into(), evidence_refs: vec![], risk: ProposalRisk::Low, status: ProposalStatus::Created, created_at: now, votes: Default::default(), vote_deadline: None, votes_requested: false, quorum_needed: 1 };
/// assert_eq!(value.quorum_needed, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    /// Stable record identifier.
    pub id: String,
    /// Persona associated with this record or request.
    pub persona_id: String,
    /// Short title for the proposed action.
    pub title: String,
    /// Reason to execute the proposal.
    pub rationale: String,
    /// Evidence identifiers supporting the proposal.
    pub evidence_refs: Vec<String>,
    /// Risk classification used by governance.
    pub risk: ProposalRisk,
    /// Current lifecycle state.
    pub status: ProposalStatus,
    /// Time the record was created.
    pub created_at: DateTime<Utc>,
    /// Votes from personas, keyed by `persona_id`.
    #[serde(default)]
    pub votes: HashMap<String, ProposalVote>,
    /// Deadline for voting.
    pub vote_deadline: Option<DateTime<Utc>>,
    /// Whether votes have been requested this cycle.
    #[serde(default)]
    pub votes_requested: bool,
    /// Quorum required, frozen at creation time to prevent drift.
    #[serde(default)]
    pub quorum_needed: usize,
}
