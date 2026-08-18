//! Construction options for [`CognitionRuntime`](super::CognitionRuntime).

use super::PersonaPolicy;

/// Runtime options for the cognition manager.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::CognitionRuntimeOptions;
/// let options = CognitionRuntimeOptions::default();
/// assert!(!options.enabled);
/// assert_eq!(options.loop_interval_ms, 2_000);
/// ```
#[derive(Debug, Clone)]
pub struct CognitionRuntimeOptions {
    pub enabled: bool,
    pub loop_interval_ms: u64,
    pub max_events: usize,
    pub max_snapshots: usize,
    pub default_policy: PersonaPolicy,
}

impl Default for CognitionRuntimeOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            loop_interval_ms: 2_000,
            max_events: 2_000,
            max_snapshots: 128,
            default_policy: PersonaPolicy::default(),
        }
    }
}
