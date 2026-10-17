//! input budget rules.
//! Model-relative input budgets shared by context derivation and compression.

use codetether_provider_core::limits::context_window_for_model;

use super::completion_limit::session_completion_max_tokens;

const SAFETY_PERCENT: usize = 90;

const PROTOCOL_OVERHEAD_TOKENS: usize = 2_048;

/// Return the safe request-input budget for `model`.
pub fn usable(model: &str) -> usize {
    for_completion(model, session_completion_max_tokens())
}
#[cfg(test)]
#[path = "input_budget_tests/mod.rs"]
mod tests;

/// Reserve the output limit actually requested, not an assumed fraction.
pub fn for_completion(model: &str, completion: usize) -> usize {
    let window = context_window_for_model(model);
    calculate(window, completion)
}

fn calculate(window: usize, completion: usize) -> usize {
    let available = window.saturating_sub(completion.saturating_add(PROTOCOL_OVERHEAD_TOKENS));
    available.saturating_mul(SAFETY_PERCENT) / 100
}

/// Resolve a zero sentinel or cap an explicit budget to model capacity.
pub fn resolve(model: &str, configured: usize) -> usize {
    let capacity = usable(model);
    if configured == 0 {
        capacity
    } else {
        configured.min(capacity)
    }
}
