//! Vertical layout for the webview: header, body, input, suggestions, status.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Webview main rectangles, including the optional suggestions row.
/// # Examples
/// ```
/// let chunks = codetether_chat_layout::webview::compute(ratatui::layout::Rect::new(0, 0, 120, 30), "", false);
/// assert!(chunks.suggestions.is_none());
/// ```
pub struct MainChunks {
    /// Header panel rectangle.
    pub header: Rect,
    /// Body panel rectangle.
    pub body: Rect,
    /// Input panel rectangle.
    pub input: Rect,
    /// Suggestions panel rectangle.
    pub suggestions: Option<Rect>,
    /// Status panel rectangle.
    pub status: Rect,
}

/// Split `area` vertically. Input height adapts to typed lines (3–6 rows)
/// and a 5-row suggestions panel appears while slash autocomplete is open,
/// matching the classic chat layout.
/// # Arguments
/// * `area` - Available terminal rectangle.
/// * `input` - Composer text used to choose input height.
/// * `suggestions_visible` - Whether to reserve the completion list.
/// # Returns
/// Rectangles for all Webview rows.
/// # Examples
/// ```
/// let chunks = codetether_chat_layout::webview::compute(ratatui::layout::Rect::new(0, 0, 120, 30), "hello", false);
/// assert!(chunks.input.height >= 3);
/// ```
pub fn compute(area: Rect, input: &str, suggestions_visible: bool) -> MainChunks {
    let input_height = (input.lines().count().max(1) as u16 + 2).clamp(3, 6);
    let mut constraints = vec![
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(input_height),
    ];
    if suggestions_visible {
        constraints.push(Constraint::Length(5));
    }
    constraints.push(Constraint::Length(super::status_height(area.width)));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);
    MainChunks {
        header: chunks[0],
        body: chunks[1],
        input: chunks[2],
        suggestions: suggestions_visible.then(|| chunks[3]),
        status: chunks[if suggestions_visible { 4 } else { 3 }],
    }
}
