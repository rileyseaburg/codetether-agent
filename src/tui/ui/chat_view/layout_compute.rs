//! Layout computation for the chat view.
//!
//! Splits the available area into an optional agent bar, messages, input,
//! optional suggestions, and status rectangles.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

use super::layout_chunks::ChatChunks;
use crate::tui::app::state::App;

/// Split `area` into agent-bar/message/input/suggestions/status rectangles.
///
/// The agent bar occupies one row when sub-agents are spawned, otherwise
/// zero rows. Input height adapts to lines typed (3–6 rows). Suggestions
/// row appears only when autocomplete is visible.
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
    let status_height = status_bar_height(area.width);
    let agent_bar_height = u16::from(super::agent_bar::presence::visible(app));
    let mut constraints = vec![
        Constraint::Length(agent_bar_height),
        Constraint::Min(8),
        Constraint::Length(input_height),
    ];
    if suggestions_visible {
        constraints.push(Constraint::Length(5));
    }
    constraints.push(Constraint::Length(status_height));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);
    let status_idx = if suggestions_visible { 4 } else { 3 };
    ChatChunks {
        agent_bar: chunks[0],
        messages: chunks[1],
        input: chunks[2],
        suggestions: suggestions_visible.then(|| chunks[3]),
        status: chunks[status_idx],
    }
}

/// Height reserved for the bottom status bar, in rows.
///
/// Stacks to 3 rows when the terminal is narrower than the
/// [`super::status::STACK_WIDTH_THRESHOLD`]; otherwise a single row.
fn status_bar_height(width: u16) -> u16 {
    if width >= super::status::STACK_WIDTH_THRESHOLD {
        1
    } else {
        3
    }
}

#[cfg(test)]
#[path = "layout_compute_tests.rs"]
mod tests;
