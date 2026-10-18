//! Right-aligned user-message bubble layout.
use crate::{
    user_rows::{border, right_pad, row},
    user_style::box_color,
};
use ratatui::text::Line;
use unicode_width::UnicodeWidthStr;

/// Wrap body text rows in a right-aligned rounded user bubble.
///
/// # Examples
///
/// ```rust
/// use codetether_message_bubbles::user_bubble;
/// let lines = user_bubble(&["hello".to_string()], 40);
/// assert_eq!(lines.len(), 3);
/// ```
///
/// # Arguments
/// * `body` - Unstyled message rows.
/// * `panel_width` - Available terminal columns.
///
/// # Returns
/// Owned bubble lines including rounded top and bottom borders.
pub fn user_bubble(body: &[String], panel_width: usize) -> Vec<Line<'static>> {
    let color = box_color();
    let content_w = body.iter().map(|s| s.width()).max().unwrap_or(0);
    let inner = (content_w + 2).min(panel_width.saturating_sub(2)).max(8);
    let mut out = vec![right_pad(border("╭", "╮", inner, color), panel_width)];
    for text in body {
        out.push(right_pad(row(text, inner, color), panel_width));
    }
    out.push(right_pad(border("╰", "╯", inner, color), panel_width));
    out
}
