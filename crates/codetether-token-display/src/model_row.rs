//! Per-model token cost and context-usage detail.
use crate::TokenDisplay;
use codetether_telemetry::{ContextLimit, TokenUsageSnapshot};
pub(crate) fn append(
    lines: &mut Vec<String>,
    display: &TokenDisplay,
    snapshot: &TokenUsageSnapshot,
) {
    let model_cost = display.calculate_cost_for_tokens(
        &snapshot.name,
        snapshot.totals.input,
        snapshot.totals.output,
    );
    lines.push(format!(
        "    {}: {} tokens ({} requests) - {}",
        snapshot.name,
        snapshot.totals.total(),
        snapshot.request_count,
        model_cost.format_currency()
    ));

    // Context limit info
    if let Some(limit) = display.get_context_limit(&snapshot.name) {
        let context = ContextLimit::new(snapshot.totals.total(), limit);
        if context.percentage >= 50.0 {
            lines.push(format!(
                "      Context: {:.1}% of {} tokens",
                context.percentage, limit
            ));
        }
    }

    crate::model_cache::append(lines, snapshot);
}
