//! PR Auto-Pilot — submit, watch CI, fix failures, respond to reviews.

use serde::{Deserialize, Serialize};

/// Lifecycle states for an auto-pilot PR.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PrLifecycle {
    Draft, CiRunning, CiFailed, Fixing, CiGreen,
    AwaitingReview, ReviewCommentsReceived, AddressingComments,
    ReadyToMerge, Merged, Failedterminal,
}

/// Configuration for a PR auto-pilot run.
#[derive(Debug, Clone)]
pub struct AutopilotConfig {
    pub issue_number: Option<u64>,
    pub branch_prefix: String,
    pub max_fix_iterations: usize,
    pub poll_interval_secs: u64,
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

/// One tick of the auto-pilot state machine.
pub fn advance_lifecycle(current: &PrLifecycle, ci_green: bool, has_review_comments: bool) -> PrLifecycle {
    match current {
        PrLifecycle::Draft => PrLifecycle::CiRunning,
        PrLifecycle::CiRunning if ci_green => PrLifecycle::CiGreen,
        PrLifecycle::CiRunning => PrLifecycle::CiFailed,
        PrLifecycle::CiFailed => PrLifecycle::Fixing,
        PrLifecycle::Fixing => PrLifecycle::CiRunning,
        PrLifecycle::CiGreen if has_review_comments => PrLifecycle::ReviewCommentsReceived,
        PrLifecycle::CiGreen => PrLifecycle::AwaitingReview,
        PrLifecycle::ReviewCommentsReceived => PrLifecycle::AddressingComments,
        PrLifecycle::AddressingComments => PrLifecycle::CiRunning,
        PrLifecycle::AwaitingReview if has_review_comments => PrLifecycle::ReviewCommentsReceived,
        PrLifecycle::AwaitingReview => PrLifecycle::ReadyToMerge,
        other => other.clone(),
    }
}
