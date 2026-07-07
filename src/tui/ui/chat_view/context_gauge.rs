//! Inline context-budget gauge for the status bar.
//!
//! Renders the percentage of the model context window consumed as a small
//! 8-cell bar. On truecolor terminals each cell is colored along a
//! green → red heat gradient (see [`context_gauge_heat`]).
//! On 8/256-color terminals a flat solid-color bar is used instead.

use ratatui::{
    style::{Color, Style},
    text::Span,
};

use crate::tui::app::state::App;
use crate::tui::ui::gradient::rgb_supported;

#[path = "context_gauge_heat.rs"]
pub mod context_gauge_heat;

const CELLS: usize = 8;

fn gauge_pct(app: &App) -> Option<usize> {
    let (used, budget) = (app.state.context_used?, app.state.context_budget?);
    if budget == 0 {
        return None;
    }
    Some(((used as f64 / budget as f64) * 100.0).clamp(0.0, 100.0) as usize)
}

/// Build a gradient context-budget gauge, or `None` if no estimate yet.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::context_gauge::context_gauge_spans;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let _maybe = context_gauge_spans(app);
/// # }
/// ```
pub fn context_gauge_spans(app: &App) -> Option<Vec<Span<'static>>> {
    let pct = gauge_pct(app)?;
    let filled = (pct * CELLS) / 100;
    if rgb_supported() {
        Some(context_gauge_heat::heat_gradient_spans(filled, CELLS, pct))
    } else {
        Some(vec![flat_gauge_span(filled, pct)])
    }
}

/// Single-span gauge for non-truecolor terminals.
pub fn context_gauge_span(app: &App) -> Option<Span<'static>> {
    let pct = gauge_pct(app)?;
    Some(flat_gauge_span((pct * CELLS) / 100, pct))
}

fn flat_gauge_span(filled: usize, pct: usize) -> Span<'static> {
    let bar: String = (0..CELLS)
        .map(|i| if i < filled { '█' } else { '░' })
        .collect();
    let color = match pct {
        0..=70 => Color::Green,
        71..=90 => Color::Yellow,
        _ => Color::Red,
    };
    Span::styled(format!("▕{bar}▏ {pct}%"), Style::default().fg(color))
}
