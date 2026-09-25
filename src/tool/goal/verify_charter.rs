//! Instructions for the second agent, chosen by the claimed transition.
//!
//! A `complete` claim gets a completion audit. A `blocked` claim gets
//! controlled opposition: an adversary whose job is to find a way past every
//! blocker, so the worker cannot stop just by declaring itself stuck.

use crate::session::tasks::GoalStatus;

const COMPLETION: &str = include_str!("verify_charter.md");
const OPPOSITION: &str = include_str!("verify_opposition.md");

/// Return the second agent's instructions for a claimed transition.
///
/// # Arguments
///
/// * `claimed` — The terminal status the worker requested.
///
/// # Returns
///
/// The controlled-opposition charter for [`GoalStatus::Blocked`], otherwise
/// the completion-audit charter.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::tasks::GoalStatus;
/// use codetether_agent::tool::goal::verify::charter;
///
/// assert!(charter(GoalStatus::Blocked).contains("controlled opposition"));
/// assert!(charter(GoalStatus::Complete).contains("completion verifier"));
/// assert!(charter(GoalStatus::Blocked).ends_with("counts as FAIL.\n"));
/// ```
pub fn charter(claimed: GoalStatus) -> &'static str {
    match claimed {
        GoalStatus::Blocked => OPPOSITION,
        GoalStatus::Complete
        | GoalStatus::Active
        | GoalStatus::Paused
        | GoalStatus::UsageLimited
        | GoalStatus::BudgetLimited => COMPLETION,
    }
}
