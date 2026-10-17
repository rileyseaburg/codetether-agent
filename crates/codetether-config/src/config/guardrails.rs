//! Cost-guardrail configuration.
//!
//! Lets users bound runaway spend on long-running agent sessions. Values
//! may come from `[guardrails]` in the config file or from the
//! `CODETETHER_COST_WARN_USD` / `CODETETHER_COST_LIMIT_USD` environment
//! variables (env takes precedence).

use serde::{Deserialize, Serialize};

/// Spending caps applied to the agentic loop.
///
/// Both fields are `None` by default — i.e. no limits. Once `warn_usd`
/// is reached the agent emits a one-shot `tracing::warn!`. Once
/// `hard_limit_usd` is reached the agent refuses further provider
/// requests and returns an error.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CostGuardrails {
    /// Warn once when cumulative session cost exceeds this USD threshold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warn_usd: Option<f64>,

    /// Hard-stop: refuse new provider requests above this USD threshold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard_limit_usd: Option<f64>,
}

impl CostGuardrails {
    /// Overlay environment-variable overrides on top of a base config.
    pub fn with_env_overrides(mut self) -> Self {
        if let Some(v) = env_f64("CODETETHER_COST_WARN_USD") {
            self.warn_usd = Some(v);
        }
        if let Some(v) = env_f64("CODETETHER_COST_LIMIT_USD") {
            self.hard_limit_usd = Some(v);
        }
        self
    }

    /// Load guardrails using env-only (no config file). Used from code
    /// paths that don't have a loaded [`crate::config::Config`] on hand.
    pub fn from_env() -> Self {
        Self::default().with_env_overrides()
    }
}

fn env_f64(key: &str) -> Option<f64> {
    std::env::var(key).ok().and_then(|v| v.trim().parse().ok())
}
