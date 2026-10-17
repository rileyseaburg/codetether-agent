//! Resource and lineage policy boundaries for a persona.

use serde::{Deserialize, Serialize};

/// Policy boundaries for a persona.
///
/// # Examples
///
/// ```rust
/// use codetether_cognition_core::PersonaPolicy;
/// let policy = PersonaPolicy::default();
/// assert_eq!(policy.max_spawn_depth, 4);
/// assert!(!policy.share_memory);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaPolicy {
    /// Maximum allowed child depth.
    pub max_spawn_depth: u32,
    /// Maximum direct children per persona.
    pub max_branching_factor: u32,
    /// Token allowance for each minute window.
    pub token_budget_per_minute: u32,
    /// Compute-time allowance for each minute window.
    pub compute_ms_per_minute: u32,
    /// Idle duration before the persona is eligible for reaping.
    pub idle_ttl_secs: u64,
    /// Whether persona memory may be shared.
    pub share_memory: bool,
    #[serde(default)]
    /// Tool names allowed by the persona policy.
    pub allowed_tools: Vec<String>,
}

impl Default for PersonaPolicy {
    fn default() -> Self {
        Self {
            max_spawn_depth: 4,
            max_branching_factor: 4,
            token_budget_per_minute: 20_000,
            compute_ms_per_minute: 10_000,
            idle_ttl_secs: 3_600,
            share_memory: false,
            allowed_tools: Vec::new(),
        }
    }
}
