//! Query planning for retrieval.

use super::{PlanIntent, RetrievalPlan, text};

/// Convert a user query into a retrieval plan.
pub(super) fn build(query: &str, budget_tokens: usize) -> RetrievalPlan {
    RetrievalPlan {
        terms: text::terms(query),
        budget_tokens,
        intent: intent(query),
    }
}

fn intent(query: &str) -> PlanIntent {
    let lower = query.to_lowercase();
    if has_any(&lower, &["error", "fail", "panic", "warning", "bug"]) {
        PlanIntent::Debug
    } else if has_any(&lower, &["function", "struct", "symbol", "where", "call"]) {
        PlanIntent::Symbol
    } else {
        PlanIntent::Summary
    }
}

fn has_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}
