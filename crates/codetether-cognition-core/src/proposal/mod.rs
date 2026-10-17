//! Proposal records, votes and governance configuration.
//!
//! [Proposal] carries frozen voting requirements and status. [SwarmGovernance]
//! supplies policy defaults; execution and vote resolution belong to the host.
mod governance;
mod kind;
mod record;
pub use governance::SwarmGovernance;
pub use kind::{ProposalRisk, ProposalStatus, ProposalVote};
pub use record::Proposal;

/// Proposal voting and deadline resolution policy.
pub mod voting;

/// Proposal creation and corresponding event records.
pub mod build;
/// Proposal title normalization.
pub mod title;

/// Verified-proposal dispatch and approval recording.
pub mod dispatch;
