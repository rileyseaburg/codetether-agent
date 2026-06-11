use ratatui::layout::{Constraint, Direction, Layout, Rect};

use super::layout_mode::ChatLayoutMode;

/// Webview layout: header(3) + body(min) + input(3) + status(1).
pub fn webview_main_chunks(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(area)
        .to_vec()
}

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

#[cfg(test)]
mod tests {
    use super::show_inspector;
    use ratatui::layout::Rect;

    #[test]
    fn inspector_requires_wide_terminal() {
        assert!(show_inspector(Rect::new(0, 0, 120, 24)));
        assert!(!show_inspector(Rect::new(0, 0, 100, 24)));
    }
}
