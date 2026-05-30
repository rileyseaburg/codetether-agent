//! Step-limit outcome tracking for non-interactive resumable runs.
//!
//! Uses a thread-local flag so that concurrent sessions in separate threads
//! (swarm, relay, server) do not race on a single global. Each CLI invocation
//! or task-fiber gets its own independent flag.
//!
//! # Semantics
//!
//! The flag starts **cleared** (`false`). When a step-limited run begins,
//! [`mark_budget_active`] sets it to `true`. At the end of the agentic loop:
//! - [`clear_budget`] is called if the run completed normally (did not hit
//!   the step limit), setting the flag back to `false`.
//! - If the flag is still `true` when the loop exits, the step budget was
//!   exhausted without calling [`clear_budget`].
//!
//! [`was_budget_exhausted`] returns `true` only when the budget was active
//! and was not cleared, meaning the loop hit the step ceiling.

use std::cell::Cell;

thread_local! {
    static BUDGET_ACTIVE: Cell<bool> = const { Cell::new(false) };
}

/// Mark a step-limited run as active. Call at the start of the agentic loop.
pub(crate) fn mark_budget_active() {
    BUDGET_ACTIVE.with(|c| c.set(true));
}

/// Clear the step-limit flag because the run completed within budget.
pub(crate) fn clear_budget() {
    BUDGET_ACTIVE.with(|c| c.set(false));
}

/// Returns `true` if the step budget was active (not cleared) after the
/// agentic loop exited, indicating the run hit the `max_steps` ceiling.
pub fn was_budget_exhausted() -> bool {
    BUDGET_ACTIVE.with(|c| c.get())
}
