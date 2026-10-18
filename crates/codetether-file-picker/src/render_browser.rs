//! File picker render browser.
use ratatui::prelude::*;

use super::types::FilePickerState;

pub fn render_browser(f: &mut ratatui::Frame, area: Rect, state: &FilePickerState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .split(area);
    super::render_list::render_list(f, chunks[0], state);
    super::render_preview::render_preview(f, chunks[1], state);
}
