use ratatui::layout::{Constraint, Direction, Layout, Rect};

use super::layout_mode::ChatLayoutMode;

/// Body: sidebar + center (+ optional inspector).
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

pub fn is_webview(mode: ChatLayoutMode) -> bool {
    mode == ChatLayoutMode::Webview
}

pub fn show_inspector(area: Rect) -> bool {
    area.width >= 118
}

/// Whether the 26-column sidebar fits beside a readable chat column.
///
/// Phone SSH clients in landscape report ~100 columns; there the sidebar
/// would squeeze chat to ~74 columns, so it is hidden below 110.
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
