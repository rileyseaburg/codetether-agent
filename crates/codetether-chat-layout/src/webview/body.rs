use ratatui::layout::{Constraint, Direction, Layout, Rect};

use super::ChatLayoutMode;

/// Lay out sidebar, center and optional inspector.
/// # Arguments
/// * `area` - Body rectangle.
/// * `show_inspector` - Whether to reserve the inspector column.
/// # Returns
/// Horizontal panel rectangles in display order.
/// # Examples
/// ```
/// assert_eq!(codetether_chat_layout::webview::webview_body_chunks(ratatui::layout::Rect::new(0, 0, 140, 30), true).len(), 3);
/// ```
pub fn webview_body_chunks(area: Rect, show_inspector: bool) -> Vec<Rect> {
    let cs = if show_inspector {
        vec![
            Constraint::Length(26),
            Constraint::Min(40),
            Constraint::Length(30),
        ]
    } else {
        vec![Constraint::Length(26), Constraint::Min(40)]
    };
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(cs)
        .split(area)
        .to_vec()
}

/// Test whether Webview is selected.
/// # Arguments
/// * `mode` - Current layout choice.
/// # Returns
/// Whether the Webview layout is active.
/// # Examples
/// ```
/// assert!(codetether_chat_layout::webview::is_webview(Default::default()));
/// ```
pub fn is_webview(mode: ChatLayoutMode) -> bool {
    mode == ChatLayoutMode::Webview
}

/// Test whether the inspector fits.
/// # Arguments
/// * `area` - Available terminal rectangle.
/// # Returns
/// True at 118 columns or wider.
/// # Examples
/// ```
/// assert!(!codetether_chat_layout::webview::show_inspector(ratatui::layout::Rect::new(0, 0, 100, 30)));
/// ```
pub fn show_inspector(area: Rect) -> bool {
    area.width >= 118
}

/// Whether the 26-column sidebar fits beside a readable chat column.
///
/// Phone SSH clients in landscape report ~100 columns; there the sidebar
/// would squeeze chat to ~74 columns, so it is hidden below 110.
/// Test whether the sidebar fits.
/// # Arguments
/// * `area` - Available terminal rectangle.
/// # Returns
/// True at 110 columns or wider.
/// # Examples
/// ```
/// assert!(codetether_chat_layout::webview::show_sidebar(ratatui::layout::Rect::new(0, 0, 120, 30)));
/// ```
pub fn show_sidebar(area: Rect) -> bool {
    area.width >= 110
}

#[cfg(test)]
mod tests {
    use super::{show_inspector, show_sidebar};
    use ratatui::layout::Rect;

    #[test]
    fn sidebar_hidden_on_phone_landscape() {
        assert!(!show_sidebar(Rect::new(0, 0, 100, 18)));
        assert!(show_sidebar(Rect::new(0, 0, 120, 24)));
    }

    #[test]
    fn inspector_requires_wide_terminal() {
        assert!(show_inspector(Rect::new(0, 0, 120, 24)));
        assert!(!show_inspector(Rect::new(0, 0, 100, 24)));
    }
}
