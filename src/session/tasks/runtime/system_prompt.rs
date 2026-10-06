//! Injection of persisted goal governance into provider system prompts.

use crate::session::tasks::{TaskLog, governance_block, state_cache};

pub(crate) fn compose(base: &str, session_id: &str) -> String {
    let base = super::super::turn_instructions::append(base.into());
    let Ok(log) = TaskLog::for_session(session_id) else {
        return base;
    };
    from_log(base, &log)
}

fn from_log(base: String, log: &TaskLog) -> String {
    let state = state_cache::load(log).unwrap_or_default();
    match governance_block(&state) {
        Some(block) => format!("{base}\n\n{block}"),
        None => base,
    }
}

#[cfg(test)]
#[path = "system_prompt_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "system_prompt_goal_tests.rs"]
mod system_prompt_goal_tests;
