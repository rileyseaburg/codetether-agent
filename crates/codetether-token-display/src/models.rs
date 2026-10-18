//! Bound the detailed model list and report additional models.
use crate::TokenDisplay;
use codetether_telemetry::TokenUsageSnapshot;
pub(crate) fn append(
    lines: &mut Vec<String>,
    display: &TokenDisplay,
    model_snapshots: &[TokenUsageSnapshot],
) {
    // Per-model breakdown
    if !model_snapshots.is_empty() {
        lines.push("  BY MODEL:".to_string());

        for snapshot in model_snapshots.iter().take(5) {
            crate::model_row::append(lines, display, snapshot);
        }

        if model_snapshots.len() > 5 {
            lines.push(format!(
                "    ... and {} more models",
                model_snapshots.len() - 5
            ));
        }
        lines.push("".to_string());
    }
}
