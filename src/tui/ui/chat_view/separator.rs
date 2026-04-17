//! Horizontal separator line between chat entries.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::tui::ui::tool_panel::separator_pattern;
use crate::tui::ui::tool_panel::RenderEntry;

/// Push a dim horizontal rule into `lines` based on the entry type.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::separator::push_separator;
/// use codetether_agent::tui::ui::tool_panel::RenderEntry;
/// let entry = RenderEntry::default();
/// let mut lines: Vec<ratatui::text::Line<'static>> = Vec::new();
/// push_separator(&mut lines, &entry, 20);
/// assert_eq!(lines.len(), 1);
/// ```
pub fn push_separator(
    lines: &mut Vec<Line<'static>>,
    entry: &RenderEntry<'_>,
    width: usize,
) {
    let sep = separator_pattern(entry);
    lines.push(Line::from(Span::styled(
        sep.repeat(width),
        Style::default().fg(Color::DarkGray).dim(),
    )));
}
