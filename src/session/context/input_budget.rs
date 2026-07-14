//! Model-relative input budgets shared by context derivation and compression.

use crate::session::helper::token::{context_window_for_model, session_completion_max_tokens};

const SAFETY_PERCENT: usize = 90;
const PROTOCOL_OVERHEAD_TOKENS: usize = 2_048;
const MAX_COMPLETION_WINDOW_FRACTION: usize = 4;

/// Return the safe request-input budget for `model`.
pub(crate) fn usable(model: &str) -> usize {
    let window = context_window_for_model(model);
    calculate(window, session_completion_max_tokens())
}

fn calculate(window: usize, completion: usize) -> usize {
    let completion = completion.min(window / MAX_COMPLETION_WINDOW_FRACTION);
    let available = window.saturating_sub(completion.saturating_add(PROTOCOL_OVERHEAD_TOKENS));
    available.saturating_mul(SAFETY_PERCENT) / 100
}

/// Resolve a zero sentinel or cap an explicit budget to model capacity.
pub(crate) fn resolve(model: &str, configured: usize) -> usize {
    let capacity = usable(model);
    if configured == 0 {
        capacity
    } else {
        configured.min(capacity)
    }
}

#[cfg(test)]
#[path = "input_budget_tests.rs"]
mod tests;
