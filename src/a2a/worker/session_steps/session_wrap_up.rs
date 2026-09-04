//! Budget wrap-up nudge for worker sessions.
//!
//! An autonomous run that keeps editing until its last step leaves
//! uncommitted work stranded in the workspace (riley/spotlessbinco#5847,
//! task d66cebc5: 27 files changed, nothing pushed, then "step budget (200)
//! exhausted"). When the remaining budget drops to the wrap-up window, inject
//! one user-role message telling the agent to stop expanding scope and commit,
//! push, and report what exists so far.

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

/// Fraction of the budget reserved for finishing up (commit, push, report).
const WRAP_UP_FRACTION: f64 = 0.15;
/// Never reserve fewer steps than this for wrap-up on budgets that allow it.
const WRAP_UP_MIN_STEPS: usize = 8;

/// Returns the step at which the wrap-up nudge should be injected.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(wrap_up_step(200), 170);
/// ```
pub(super) fn wrap_up_step(max_steps: usize) -> usize {
    let reserve = ((max_steps as f64) * WRAP_UP_FRACTION).ceil() as usize;
    let reserve = reserve.max(WRAP_UP_MIN_STEPS).min(max_steps.saturating_sub(1));
    max_steps.saturating_sub(reserve).max(1)
}

/// Injects the wrap-up nudge when `step` is the wrap-up step for `max_steps`.
/// Returns `true` when a message was added.
pub(super) fn maybe_inject(session: &mut Session, step: usize, max_steps: usize) -> bool {
    if max_steps < 2 || step != wrap_up_step(max_steps) {
        return false;
    }
    let remaining = max_steps.saturating_sub(step);
    tracing::warn!(step, max_steps, remaining, "Injecting step-budget wrap-up nudge");
    session.add_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: format!(
                "SYSTEM NOTICE: only {remaining} model steps remain in this task's budget \
                 of {max_steps}. Stop expanding scope now. In the remaining steps: \
                 (1) make sure the code compiles/passes the narrowest check you already ran, \
                 (2) `git add` and commit everything you changed with a descriptive message, \
                 (3) push the branch and open or update the pull request if the task asked for one, \
                 (4) post the commit/PR link and a short status back to the issue or PR. \
                 Unfinished items must be listed in the PR description as follow-ups, not attempted."
            ),
        }],
    });
    true
}

#[cfg(test)]
mod tests {
    use super::wrap_up_step;

    #[test]
    fn wrap_up_reserves_fraction_with_floor() {
        assert_eq!(wrap_up_step(200), 170);
        assert_eq!(wrap_up_step(100), 85);
        assert_eq!(wrap_up_step(20), 12);
        assert_eq!(wrap_up_step(10), 2);
        assert_eq!(wrap_up_step(2), 1);
        assert_eq!(wrap_up_step(1), 1);
    }
}
