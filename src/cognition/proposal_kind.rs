//! Proposal risk, status, and vote classification.

use serde::{Deserialize, Serialize};

/// Proposal risk level.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::ProposalRisk;
/// assert_ne!(ProposalRisk::Low, ProposalRisk::Critical);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalRisk {
    Low,
    Medium,
    High,
    Critical,
}

/// Proposal lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Created,
    Verified,
    Rejected,
    Executed,
}

/// A vote on a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalVote {
    Approve,
    Reject,
    Veto,
    Abstain,
}
