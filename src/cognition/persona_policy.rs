//! Resource and lineage policy boundaries for a persona.

use serde::{Deserialize, Serialize};

/// Policy boundaries for a persona.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::PersonaPolicy;
/// let policy = PersonaPolicy::default();
/// assert_eq!(policy.max_spawn_depth, 4);
/// assert!(!policy.share_memory);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaPolicy {
    pub max_spawn_depth: u32,
    pub max_branching_factor: u32,
    pub token_budget_per_minute: u32,
    pub compute_ms_per_minute: u32,
    pub idle_ttl_secs: u64,
    pub share_memory: bool,
    #[serde(default)]
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
