//! Rendering for model-visible automatic goal continuation.

use crate::session::tasks::Goal;

const TEMPLATE: &str = include_str!("continuation.md");

pub(super) fn render(goal: &Goal) -> String {
    let budget = goal.token_budget.map_or_else(|| "none".into(), |value| value.to_string());
    let remaining = goal.token_budget.map_or_else(|| "unbounded".into(),
        |limit| limit.saturating_sub(goal.tokens_used).max(0).to_string());
    TEMPLATE.replace("{{ objective }}", &escape(&goal.objective))
        .replace("{{ tokens_used }}", &goal.tokens_used.to_string())
        .replace("{{ token_budget }}", &budget)
        .replace("{{ remaining_tokens }}", &remaining)
}

fn escape(input: &str) -> String {
    input.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
