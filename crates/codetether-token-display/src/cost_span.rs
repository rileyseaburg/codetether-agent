//! Colorize session cost using the existing cost-guard severity.
use codetether_telemetry::CostEstimate;
use codetether_theme::Theme;
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};
pub(crate) fn append(spans: &mut Vec<Span<'static>>, theme: &Theme, session_cost: CostEstimate) {
    // Cost — colorized based on the configured cost guardrails so
    // users get an immediate visual when they cross warn / hard limit
    // thresholds (see `CODETETHER_COST_WARN_USD` /
    // `CODETETHER_COST_LIMIT_USD`).
    let cost_style = match codetether_cost_guard::cost_guard_level() {
        codetether_cost_guard::CostGuardLevel::OverLimit => {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        }
        codetether_cost_guard::CostGuardLevel::OverWarn => Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        codetether_cost_guard::CostGuardLevel::Ok => {
            Style::default().fg(theme.timestamp_color.to_color())
        }
    };
    spans.push(Span::styled(
        format!(" Cost: {} ", session_cost.format_smart()),
        cost_style,
    ));
}
