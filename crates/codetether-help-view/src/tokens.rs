//! Help tokens content and presentation.
use crate::blank::blank;
use codetether_token_display::TokenDisplay;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(lines: &mut Vec<Line<'static>>, token_display: &TokenDisplay) {
    let global = codetether_telemetry::TOKEN_USAGE.global_snapshot();
    if global.total.total() > 0 {
        let _cost = token_display.calculate_cost_for_tokens("gpt-4o", global.input, global.output);
        lines.push(Line::from(vec![
            Span::styled("  Tokens: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} total", global.total.total()),
                Style::default().fg(Color::Yellow),
            ),
        ]));
        lines.push(blank());
    }
}
