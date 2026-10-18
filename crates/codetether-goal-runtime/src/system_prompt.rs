//! Injection of persisted goal governance into provider system prompts.

use crate::Journal;
use crate::state_cache;
use codetether_goal_core::{TaskLog, governance_block};

/// Append the turn contract and any readable goal governance to a base prompt.
/// # Arguments
/// `base` is the original prompt; `session_id` selects the journal.
/// # Returns
/// Returns the full prompt; journal errors retain only the shared turn contract.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, compose};
/// # fn example<H: Journal>() {
/// let value = compose::<H>("Project rules", "session-1234");
/// # }
/// ```
pub fn compose<H: Journal>(base: &str, session_id: &str) -> String {
    let base = crate::turn_instructions::append(base.into());
    let Ok(log) = H::for_session(session_id) else {
        return base;
    };
    from_log(base, &log)
}

/// Append readable journal governance to an already-prepared base prompt.
/// # Arguments
/// `base` includes existing instructions; `log` is an explicitly resolved journal.
/// # Returns
/// Returns the prompt with governance, retaining the base on read errors.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, compose_from_log};
/// # fn example(log: &codetether_goal_core::TaskLog) {
/// let value = compose_from_log("Project rules".into(), log);
/// # }
/// ```
pub fn from_log(base: String, log: &TaskLog) -> String {
    let state = state_cache::load(log).unwrap_or_default();
    match governance_block(&state) {
        Some(block) => format!("{base}\n\n{block}"),
        None => base,
    }
}

#[cfg(test)]
#[path = "system_prompt_goal_tests.rs"]
mod system_prompt_goal_tests;
