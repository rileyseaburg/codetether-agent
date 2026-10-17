//! step report for the planning tools.
use super::*;
pub(super) fn render(plan: &Plan) -> String {
    plan.steps
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let icon = if s.completed { "✓" } else { "○" };
            let notes = s
                .notes
                .as_ref()
                .map(|n| format!(" [{}]", n))
                .unwrap_or_default();
            format!("  {} {}. {}{}", icon, i + 1, s.description, notes)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
