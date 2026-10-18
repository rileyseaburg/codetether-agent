//! Relay status, final handoff and planning summary.
use crate::Summary;
use codetether_relay_plan::truncate_with_ellipsis;
pub(super) fn build(input: &Summary<'_>) -> String {
    let status = input.status;
    let mut summary = format!(
        "Autochat complete ({status}) — relay {} with {} agents over {} turns.\n\nFinal relay handoff:\n{}",
        input.relay_id,
        input.agent_count,
        input.turns,
        truncate_with_ellipsis(input.final_handoff, 4_000)
    );
    if let Some(note) = &input.failure {
        summary.push_str(&format!("\n\nFailure detail: {note}"));
    }
    if input.planner_used {
        summary.push_str("\n\nTeam planning: model-organized profiles.");
    } else {
        summary.push_str("\n\nTeam planning: fallback self-organizing profiles.");
    }
    summary
}
