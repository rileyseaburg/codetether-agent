//! Shared forage alignment rules.
use crate::tokenize::tokenize_for_alignment;
pub(super) fn moonshot_alignment_score(context: &str, goals: &[String]) -> f64 {
    if goals.is_empty() {
        return 0.0;
    }

    let context_tokens = tokenize_for_alignment(context);
    if context_tokens.is_empty() {
        return 0.0;
    }

    goals
        .iter()
        .map(|goal| {
            let goal_tokens = tokenize_for_alignment(goal);
            if goal_tokens.is_empty() {
                return 0.0;
            }
            let overlap = goal_tokens.intersection(&context_tokens).count() as f64;
            overlap / goal_tokens.len() as f64
        })
        .fold(0.0, f64::max)
}
