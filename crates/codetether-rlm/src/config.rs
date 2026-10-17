//! RLM serialized runtime configuration.

use serde::{Deserialize, Serialize};
mod config_defaults;
mod default;

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
#[cfg(test)]
mod tests;
