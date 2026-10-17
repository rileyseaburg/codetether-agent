//! enter report for the planning tools.
use super::*;
pub(super) fn render(p: &EnterParams) -> String {
    format!(
        "📋 Plan Mode Activated\n\nGoal: {}\n\nSteps:\n{}",
        p.goal,
        p.steps
            .iter()
            .enumerate()
            .map(|(i, s)| format!("  {}. {}", i + 1, s))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
