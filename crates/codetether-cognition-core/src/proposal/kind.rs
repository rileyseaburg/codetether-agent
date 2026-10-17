//! Proposal risk, status, and vote classification.

use serde::{Deserialize, Serialize};

/// Proposal risk level.
///
/// # Examples
///
/// ```rust
/// use codetether_cognition_core::ProposalRisk;
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
///
/// Variants: Created, Verified, Rejected, Executed.
///
/// # Examples
/// ```
/// use codetether_cognition_core::ProposalStatus;
/// match ProposalStatus::Created {
///     ProposalStatus::Created => {},
///     other => panic!("unexpected value: {other:?}"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Created,
    Verified,
    Rejected,
    Executed,
}

/// A vote on a proposal.
///
/// Variants: Approve, Reject, Veto, Abstain.
///
/// # Examples
/// ```
/// use codetether_cognition_core::ProposalVote;
/// match ProposalVote::Approve {
///     ProposalVote::Approve => {},
///     other => panic!("unexpected value: {other:?}"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalVote {
    Approve,
    Reject,
    Veto,
    Abstain,
}
