//! Governance rules for swarm proposal voting.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ProposalRisk;

/// Governance rules for swarm voting.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::SwarmGovernance;
/// let rules = SwarmGovernance::default();
/// assert_eq!(rules.vote_timeout_secs, 300);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmGovernance {
    pub quorum_fraction: f32,
    pub required_approvers_by_role: HashMap<ProposalRisk, Vec<String>>,
    pub veto_roles: Vec<String>,
    pub vote_timeout_secs: u64,
}

impl Default for SwarmGovernance {
    fn default() -> Self {
        Self {
            quorum_fraction: 0.5,
            required_approvers_by_role: HashMap::new(),
            veto_roles: Vec::new(),
            vote_timeout_secs: 300,
        }
    }
}
