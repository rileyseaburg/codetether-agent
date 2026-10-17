//! incremental budget config rules.
//! Session and environment resolution for incremental context budgets.

use crate::derivation::DerivePolicy;

pub fn resolve(persisted: DerivePolicy, model: &str) -> usize {
    let persisted = match persisted {
        DerivePolicy::Incremental { budget_tokens } => budget_tokens,
        _ => 0,
    };
    let configured = std::env::var("CODETETHER_CONTEXT_INCREMENTAL_BUDGET_TOKENS")
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|tokens| *tokens > 0)
        .unwrap_or(persisted);
    crate::derivation::input_budget::resolve(model, configured)
}
