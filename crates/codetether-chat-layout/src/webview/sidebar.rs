//! Sidebar geometry for workspace and recent-session panels.
use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Divide a sidebar into workspace and recent-session rectangles.
/// # Arguments
/// * `area` - Available sidebar rectangle.
/// # Returns
/// Two rectangles, with workspace first and recent sessions second.
/// # Examples
/// ```
/// let panels = codetether_chat_layout::webview::sidebar_chunks(ratatui::layout::Rect::new(0, 0, 26, 36));
/// assert_eq!(panels.len(), 2);
/// assert_eq!(panels[1].height, 12);
/// ```
pub fn sidebar_chunks(area: Rect) -> Vec<Rect> {
    let recent_height = recent_sessions_height(area.height);
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(recent_height)])
        .split(area)
        .to_vec()
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
