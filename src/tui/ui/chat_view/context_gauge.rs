//! Inline context-budget gauge for the status bar.
//!
//! Renders the percentage of the model context window consumed as a small
//! 8-cell bar. Reuses `context_used`/`context_budget` already tracked on
//! [`AppState`], so it adds no extra computation and only changes (triggering
//! a redraw) when the percentage moves.

use ratatui::{
    style::{Color, Style},
    text::Span,
};

use crate::tui::app::state::App;

const CELLS: usize = 8;

/// Build an inline context-budget gauge span, or `None` if no estimate yet.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::context_gauge::context_gauge_span;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let _maybe = context_gauge_span(app);
/// # }
/// ```
pub fn context_gauge_span(app: &App) -> Option<Span<'static>> {
    let (Some(used), Some(budget)) = (app.state.context_used, app.state.context_budget) else {
        return None;
    };
    if budget == 0 {
        return None;
    }
    let pct = ((used as f64 / budget as f64) * 100.0).clamp(0.0, 100.0) as usize;
    let filled = (pct * CELLS) / 100;
    let bar: String = (0..CELLS)
        .map(|i| if i < filled { '█' } else { '░' })
        .collect();
    let color = match pct {
        0..=70 => Color::Green,
        71..=90 => Color::Yellow,
        _ => Color::Red,
    };
    Some(Span::styled(
        format!("▕{bar}▏ {pct}%"),
        Style::default().fg(color),
    ))
}
