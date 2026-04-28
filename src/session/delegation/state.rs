//! `DelegationState` core data structure and key construction.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::beta::BetaPosterior;
use super::config::DelegationConfig;
use super::env::env_enabled_override;
use crate::session::relevance::Bucket;

/// Per-session CADMAS-CTX sidecar.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DelegationState {
    /// Posteriors keyed by `"agent|skill|difficulty|dependency|tool_use"`.
    #[serde(default)]
    pub beliefs: BTreeMap<String, BetaPosterior>,
    /// Runtime configuration.
    #[serde(default)]
    pub config: DelegationConfig,
}

impl DelegationState {
    /// Create a fresh state seeded with the supplied config.
    pub fn with_config(config: DelegationConfig) -> Self {
        Self {
            beliefs: BTreeMap::new(),
            config,
        }
    }

    /// Whether CADMAS-CTX routing is active.
    pub fn enabled(&self) -> bool {
        env_enabled_override().unwrap_or(self.config.enabled)
    }

    /// Encode `(agent, skill, bucket)` as a flat string key.
    pub fn key(agent: &str, skill: &str, bucket: Bucket) -> String {
        format!(
            "{agent}|{skill}|{}|{}|{}",
            bucket.difficulty.as_str(),
            bucket.dependency.as_str(),
            bucket.tool_use.as_str(),
        )
    }
}
