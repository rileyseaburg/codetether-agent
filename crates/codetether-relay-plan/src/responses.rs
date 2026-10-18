//! Planner response and spawn decision schemas.
use crate::PlannedRelayProfile;
use serde::Deserialize;

/// Optional list of planned relay specialists.
/// # Examples
/// ```
/// let response: codetether_relay_plan::PlannedRelayResponse = serde_json::from_str("{}").unwrap();
/// assert!(response.profiles.is_empty());
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct PlannedRelayResponse {
    #[serde(default)]
    /// Candidate specialists in planner response order.
    pub profiles: Vec<PlannedRelayProfile>,
}

/// Planner decision about adding another relay specialist.
/// # Examples
/// ```
/// let decision: codetether_relay_plan::RelaySpawnDecision = serde_json::from_str("{}").unwrap();
/// assert!(!decision.spawn);
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct RelaySpawnDecision {
    #[serde(default)]
    /// Whether the planner requests another specialist.
    pub spawn: bool,
    #[serde(default)]
    /// Planner explanation for the spawn decision.
    pub reason: String,
    #[serde(default)]
    /// Candidate specialist when supplied by the planner.
    pub profile: Option<PlannedRelayProfile>,
}
