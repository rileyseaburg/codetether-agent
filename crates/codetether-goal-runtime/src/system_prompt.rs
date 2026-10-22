//! Injection of persisted goal governance into provider system prompts.

use crate::Journal;
use crate::state_cache;
use codetether_goal_core::{TaskLog, governance_block};

/// Append the turn contract and any readable goal governance to a base prompt.
/// # Arguments
/// `base` is the original prompt; `session_id` selects the journal.
/// # Returns
/// Returns the full prompt; journal errors add an explicit recovery instruction.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, compose};
/// # fn example<H: Journal>() {
/// let value = compose::<H>("Project rules", "session-1234");
/// # }
/// ```
pub fn compose<H: Journal>(base: &str, session_id: &str) -> String {
    let (base, _) = codetether_provider_core::runtime_context::split(base);
    let base = crate::turn_instructions::append(base);
    let log = match H::for_session(session_id) {
        Ok(log) => log,
        Err(error) => return unavailable(base, &error),
    };
    from_log(base, &log)
}

/// Append readable journal governance to an already-prepared base prompt.
/// # Arguments
/// `base` includes existing instructions; `log` is an explicitly resolved journal.
/// # Returns
/// Returns the prompt with governance, or an explicit recovery instruction.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, compose_from_log};
/// # fn example(log: &codetether_goal_core::TaskLog) {
/// let value = compose_from_log("Project rules".into(), log);
/// # }
/// ```
pub fn from_log(base: String, log: &TaskLog) -> String {
    let (base, _) = codetether_provider_core::runtime_context::split(&base);
    let state = match state_cache::load(log) {
        Ok(state) => state,
        Err(error) => return unavailable(base, &error),
    };
    match governance_block(&state) {
        Some(block) => codetether_provider_core::runtime_context::append(&base, &block),
        None => base,
    }
}

fn unavailable(base: String, error: &anyhow::Error) -> String {
    tracing::error!(error = %error, "Goal governance could not be loaded");
    codetether_provider_core::runtime_context::append(
        &base,
        "GOAL_GOVERNANCE_UNAVAILABLE: Persisted goal and hold state could not be read. Do not infer that the goal is absent, completed, or unrestricted. Report the persistence failure and recover the journal before continuing goal work or changing goal state.",
    )
}

#[cfg(test)]
#[path = "system_prompt_goal_tests.rs"]
mod system_prompt_goal_tests;
