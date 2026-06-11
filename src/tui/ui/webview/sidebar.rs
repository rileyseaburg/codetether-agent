use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::tui::app::state::App;

pub fn render_webview_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let recent_height = recent_sessions_height(area.height);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(recent_height)])
        .split(area);
    super::workspace_panel::render(f, app, chunks[0]);
    super::sessions_panel::render(f, app, chunks[1]);
}

fn recent_sessions_height(height: u16) -> u16 {
    if height >= 30 {
        12
    } else {
        height.saturating_div(3).max(8)
    }
}

#[cfg(test)]
mod tests {
    use super::recent_sessions_height;

    #[test]
    fn recent_sessions_panel_has_stable_height() {
        assert_eq!(recent_sessions_height(36), 12);
        assert_eq!(recent_sessions_height(18), 8);
    }
}
