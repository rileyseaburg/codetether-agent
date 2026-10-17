//! Shared forage hits rules.
use crate::tokenize::tokenize_for_alignment;
pub(super) fn matching_moonshots(context: &str, goals: &[String]) -> Vec<String> {
    if goals.is_empty() {
        return Vec::new();
    }
    let context_tokens = tokenize_for_alignment(context);
    let mut hits = goals
        .iter()
        .filter_map(|goal| {
            let goal_tokens = tokenize_for_alignment(goal);
            if goal_tokens.is_empty() {
                return None;
            }
            let overlap = goal_tokens.intersection(&context_tokens).count();
            if overlap == 0 {
                None
            } else {
                Some((overlap, goal.clone()))
            }
        })
        .collect::<Vec<_>>();
    hits.sort_by(|a, b| b.0.cmp(&a.0));
    hits.into_iter().take(3).map(|(_, goal)| goal).collect()
}
