//! Layout computation for the chat view.
//!
//! Splits the available area into an optional agent bar, messages, input,
//! optional suggestions, and status rectangles.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

use super::{layout_chunks::ChatChunks, status_height::status_bar_height};

/// Split `area` into agent-bar/message/input/suggestions/status rectangles.
///
/// The agent bar occupies one row when sub-agents are spawned, otherwise
/// zero rows. Input height adapts to lines typed (3–6 rows). Suggestions
/// row appears only when autocomplete is visible.
///
/// # Arguments
/// * `area` - Available screen rectangle.
/// * `input` - Composer text used to calculate height.
/// * `suggestions_visible` - Whether to reserve the completion list.
/// * `agent_bar_visible` - Whether to reserve the agent bar.
///
/// # Returns
/// Rectangles for the complete chat layout.
///
/// # Examples
/// ```
/// let chunks = codetether_chat_layout::compute_chat_chunks(
///     ratatui::layout::Rect::new(0, 0, 80, 24), "hello", false, false);
/// assert_eq!(chunks.messages.width, 80);
/// ```
pub fn compute_chat_chunks(
    area: Rect,
    input: &str,
    suggestions_visible: bool,
    agent_bar_visible: bool,
) -> ChatChunks {
    let input_lines_count = input.lines().count().max(1);
    let input_height = (input_lines_count as u16 + 2).clamp(3, 6);
    let status_height = status_bar_height(area.width);
    let agent_bar_height = u16::from(agent_bar_visible);
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
