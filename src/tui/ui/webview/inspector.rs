use ratatui::{prelude::*, widgets::*};

use crate::tui::app::state::App;

pub fn render_webview_inspector(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Inspector ");
    f.render_widget(
        Paragraph::new(super::inspector_lines::build(app)).block(block),
        area,
    );
}
