//! Reviewer-agent settings for the approval overlay.
//!
//! When enabled, every file-mutating approval spawns an ephemeral read-only
//! agent that inspects the proposed change against the session goal and
//! reports a verdict beside the LSP panel. `advise` never decides for the
//! human; it only annotates.

use serde::{Deserialize, Serialize};

/// `[review]` table of the configuration file.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ReviewConfig {
    /// `off` (default) or `advise`.
    #[serde(default)]
    pub mode: ReviewMode,
    /// `provider/model` for the reviewer; defaults to the session model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Maximum agent-loop steps before the reviewer must conclude.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_steps: Option<usize>,
    /// Wall-clock cap in seconds; expiry yields an `escalate` verdict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,
}

/// How much authority the reviewer has.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewMode {
    /// No reviewer is spawned.
    #[default]
    Off,
    /// Reviewer annotates the approval; the human still decides.
    Advise,
}

impl ReviewConfig {
    pub fn enabled(&self) -> bool {
        self.mode != ReviewMode::Off
    }

    pub fn max_steps(&self) -> usize {
        self.max_steps.unwrap_or(12)
    }

    pub fn timeout_secs(&self) -> u64 {
        self.timeout_secs.unwrap_or(90)
    }
}
