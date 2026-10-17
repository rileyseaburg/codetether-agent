//! PR autopilot configuration and defaults.
use super::*;

/// Configuration for a PR auto-pilot run.
#[derive(Debug, Clone)]
/// PR AutopilotConfig data.
///
/// # Examples
/// ```
/// let config = codetether_github_pr::autopilot::AutopilotConfig::default();
/// assert!(!config.auto_merge);
/// ```
pub struct AutopilotConfig {
    /// issue number metadata.
    pub issue_number: Option<u64>,
    /// branch prefix metadata.
    pub branch_prefix: String,
    /// max fix iterations metadata.
    pub max_fix_iterations: usize,
    /// poll interval secs metadata.
    pub poll_interval_secs: u64,
    /// auto merge metadata.
    pub auto_merge: bool,
}

impl Default for AutopilotConfig {
    fn default() -> Self {
        Self {
            issue_number: None,
            branch_prefix: "autopilot".to_string(),
            max_fix_iterations: 5,
            poll_interval_secs: 30,
            auto_merge: false,
        }
    }
}
