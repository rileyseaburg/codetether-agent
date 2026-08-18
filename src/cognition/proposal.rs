//! Proposal contract: think first, execute through governance gates.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ProposalRisk, ProposalStatus, ProposalVote};

/// A proposed action awaiting verification, quorum, and execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub persona_id: String,
    pub title: String,
    pub rationale: String,
    pub evidence_refs: Vec<String>,
    pub risk: ProposalRisk,
    pub status: ProposalStatus,
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
