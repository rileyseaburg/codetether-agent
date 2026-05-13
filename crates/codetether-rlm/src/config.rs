//! RLM configuration type and defaults.

use serde::{Deserialize, Serialize};

/// RLM configuration.
///
/// # Examples
///
/// ```rust
/// use codetether_rlm::RlmConfig;
///
/// let cfg = RlmConfig::default();
/// assert_eq!(cfg.mode, "auto");
/// assert_eq!(cfg.max_iterations, 15);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmConfig {
    /// Mode: "auto", "off", or "always"
    #[serde(default = "config_defaults::default_mode")]
    pub mode: String,
    /// Threshold ratio of context window to trigger RLM (0.0-1.0)
    #[serde(default = "config_defaults::default_threshold")]
    pub threshold: f64,
    /// Maximum iterations for RLM processing.
    #[serde(default = "config_defaults::default_max_iterations")]
    pub max_iterations: usize,
    /// Maximum recursive sub-calls
    #[serde(default = "config_defaults::default_max_subcalls")]
    pub max_subcalls: usize,
    /// Preferred runtime: "rust", "bun", or "python"
    #[serde(default = "config_defaults::default_runtime")]
    pub runtime: String,
    /// Model reference for root processing (`provider/model` or bare model).
    pub root_model: Option<String>,
    /// Model reference for subcalls (`provider/model` or bare model).
    pub subcall_model: Option<String>,
    /// Message count trigger for RLM compaction. `0` disables.
    #[serde(default = "config_defaults::default_history_trigger_messages")]
    pub history_trigger_messages: usize,
}

mod config_defaults {
    pub fn default_mode() -> String { "auto".into() }
    pub fn default_threshold() -> f64 { 0.35 }
    pub fn default_max_iterations() -> usize { 15 }
    pub fn default_max_subcalls() -> usize { 50 }
    pub fn default_runtime() -> String { "rust".into() }
    pub fn default_history_trigger_messages() -> usize { 0 }
}

impl Default for RlmConfig {
    fn default() -> Self {
        use config_defaults::*;
        Self {
            mode: default_mode(),
            threshold: default_threshold(),
            max_iterations: default_max_iterations(),
            max_subcalls: default_max_subcalls(),
            runtime: default_runtime(),
            root_model: None,
            subcall_model: None,
            history_trigger_messages: default_history_trigger_messages(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RlmConfig;

    #[test]
    fn default_history_trigger_is_disabled() {
        assert_eq!(RlmConfig::default().history_trigger_messages, 0);
    }
}
