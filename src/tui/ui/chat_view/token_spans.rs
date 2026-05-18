//! Push token-usage spans from [`TokenDisplay`] into a span buffer.

use ratatui::style::{Color, Style};
use ratatui::text::Span;

use crate::tui::app::state::App;
use crate::tui::theme::Theme;
use crate::tui::theme_utils::validate_theme;
use crate::tui::token_display::TokenDisplay;

/// Append token-usage spans by cloning `Cow` content to `'static`.
pub fn push_token_spans(spans: &mut Vec<Span<'static>>) {
    let theme = validate_theme(&Theme::default());
    let token_display = TokenDisplay::new();
    let token_line = token_display.create_status_bar(&theme);
    for s in token_line.spans {
        spans.push(Span::styled(s.content.into_owned(), s.style));
    }
}

/// Append a `⚡N tok/s` live throughput badge while a turn is streaming.
/// Yellow under 10 tok/s (likely stalled), cyan above. No-op when idle.
pub fn push_throughput_span(spans: &mut Vec<Span<'static>>, app: &App) {
    let Some(tps) = app.state.streaming_tok_per_sec() else {
        return;
    };
    if !app.state.processing {
        return;
    }
    spans.push(Span::raw(" | "));
    let color = if tps < 10.0 {
        Color::Yellow
    } else {
        Color::Cyan
    };
    spans.push(Span::styled(
        format!("⚡{:.0} tok/s", tps),
        Style::default().fg(color),
    ));
}

/// Append a `ctx:NN%` badge built from the most recent
/// [`SessionEvent::TokenEstimate`](crate::session::SessionEvent::TokenEstimate).
/// No-op when no estimate has been observed for the session yet.
pub fn push_context_budget_span(spans: &mut Vec<Span<'static>>, app: &App) {
    let (Some(used), Some(budget)) = (app.state.context_used, app.state.context_budget) else {
        return;
    };
    if budget == 0 {
        return;
    }
    let pct = ((used as f64 / budget as f64) * 100.0) as usize;
    let color = if pct > 90 {
        Color::Red
    } else if pct > 70 {
        Color::Yellow
    } else {
        Color::Green
    };
    spans.push(Span::raw(" | "));
    spans.push(Span::styled(
        format!("ctx:{pct}%"),
        Style::default().fg(color),
    ));
}
