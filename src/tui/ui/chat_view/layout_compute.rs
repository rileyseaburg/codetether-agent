//! Layout computation for the chat view.
//!
//! Splits the available area into messages, input, optional suggestions,
//! and status rectangles.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

use super::layout_chunks::ChatChunks;
use crate::tui::app::state::App;

/// Split `area` into message/input/suggestions/status rectangles.
///
/// Input height adapts to lines typed (3–6 rows). Suggestions row appears
/// only when autocomplete is visible.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::layout_compute::compute_chat_chunks;
/// # fn d(a:&codetether_agent::tui::app::state::App){ let c = compute_chat_chunks(ratatui::layout::Rect::new(0,0,80,24), a); assert!(c.messages.width>0); }
/// ```
pub fn compute_chat_chunks(area: Rect, app: &App) -> ChatChunks {
    let suggestions_visible = app.state.slash_suggestions_visible();
    let input_lines_count = app.state.input.lines().count().max(1);
    let input_height = (input_lines_count as u16 + 2).clamp(3, 6);
    let status_height = status_bar_height(area.width, app);
    let constraints: &[Constraint] = if suggestions_visible {
        &[
            Constraint::Min(8),
            Constraint::Length(input_height),
            Constraint::Length(5),
            Constraint::Length(status_height),
        ]
    } else {
        &[
            Constraint::Min(8),
            Constraint::Length(input_height),
            Constraint::Length(status_height),
        ]
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);
    ChatChunks {
        messages: chunks[0],
        input: chunks[1],
        suggestions: suggestions_visible.then(|| chunks[2]),
        status: chunks[if suggestions_visible { 3 } else { 2 }],
    }
}

/// Height reserved for the bottom status bar, in rows.
///
/// Computes the actual number of packed status lines so every badge
/// stays visible; on wide terminals this collapses to a single row.
fn status_bar_height(width: u16, app: &App) -> u16 {
    let label = app.state.session_id.as_deref().unwrap_or("new").to_string();
    let lines = super::status::build_status_lines(app, &label, width);
    (lines.len() as u16).max(1)
}
