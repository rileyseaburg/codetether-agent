//! Cap on consecutive verifier rejections before escalating to the user.
//!
//! Without a cap a worker could re-submit reworded evidence until some
//! verifier run passes. After [`max_attempts`] consecutive rejections for a
//! goal, the verifier is no longer run: the goal is paused and the user must
//! review it and `/goal resume`, which grants a fresh set of attempts.

use crate::VerdictRecord;

/// Environment variable overriding the rejection cap.
pub const MAX_ATTEMPTS_ENV: &str = "CODETETHER_GOAL_VERIFY_MAX_ATTEMPTS";

/// Maximum consecutive rejections allowed (default 5, minimum 1).
/// # Returns
/// Configured positive cap, falling back to five for absent or invalid values.
/// # Examples
/// ```
/// assert!(codetether_goal_verification::attempt_cap::max_attempts() >= 1);
/// ```
pub fn max_attempts() -> usize {
    std::env::var(MAX_ATTEMPTS_ENV)
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(5)
}

#[cfg(test)]
#[path = "attempt_cap_tests.rs"]
mod tests;

/// Rejections for `goal_id` since its last pass or escalation.
///
/// # Examples
///
/// ```rust
/// use codetether_goal_verification::attempt_cap::rejections_since_reset;
/// use codetether_goal_verification::VerdictRecord;
///
/// let fail = VerdictRecord::new("g", "complete", false, "m", "r");
/// let pass = VerdictRecord::new("g", "complete", true, "m", "r");
/// let other = VerdictRecord::new("h", "complete", false, "m", "r");
/// let log = [fail.clone(), pass, fail.clone(), other, fail];
/// assert_eq!(rejections_since_reset(&log, "g"), 2);
/// ```
/// # Arguments
/// * `log` - Verdicts in chronological order.
/// * `goal_id` - Goal whose substantive rejections are counted.
/// # Returns
/// Rejections after the latest pass/escalation, excluding unavailable attempts.
pub fn rejections_since_reset(log: &[VerdictRecord], goal_id: &str) -> usize {
    log.iter()
        .rev()
        .filter(|r| r.goal_id == goal_id)
        .take_while(|r| !r.passed && !r.escalated)
        .filter(|r| !r.unavailable)
        .count()
}
