//! Horizontal separator line between chat entries.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::RenderEntry;
use crate::tui::ui::tool_panel::separator_pattern;
use codetether_theme::gradient::rgb_supported;
use codetether_tui_style::gradient_rule::gradient_rule;

/// Gradient endpoints for a subtle cyan → charcoal fade.
const FADE_FROM: (u8, u8, u8) = (0, 130, 150);
const FADE_TO: (u8, u8, u8) = (45, 45, 55);

/// Push a horizontal rule into `lines` based on the entry type.
///
/// On truecolor terminals the rule fades cyan → charcoal left-to-right;
/// otherwise it falls back to a dim dark-gray line.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_tool_panel::push_separator;
/// use codetether_tool_panel::RenderEntry;
/// let entry = RenderEntry::default();
/// let mut lines: Vec<ratatui::text::Line<'static>> = Vec::new();
/// push_separator(&mut lines, &entry, 20);
/// assert_eq!(lines.len(), 1);
/// ```
///
/// # Arguments
/// * `lines` - Destination transcript rows.
/// * `entry` - Entry determining the separator glyph.
/// * `width` - Separator width.
///
/// # Returns
/// Appends one styled separator line.
pub fn push_separator(lines: &mut Vec<Line<'static>>, entry: &RenderEntry<'_>, width: usize) {
    let sep = separator_pattern(entry);
    if rgb_supported() {
        lines.push(gradient_rule(sep, width, FADE_FROM, FADE_TO));
        return;
    }
    lines.push(Line::from(Span::styled(
        sep.repeat(width),
        Style::default().fg(Color::DarkGray).dim(),
    )));
}
