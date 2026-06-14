//! Shared helpers for webview render tests.

use ratatui::{Terminal, backend::TestBackend};

use crate::tui::app::state::App;

pub(super) fn buffer_text(terminal: &Terminal<TestBackend>) -> String {
    let buf = terminal.backend().buffer();
    let area = *buf.area();
    let mut text = String::new();
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            text.push_str(buf[(x, y)].symbol());
        }
        text.push('\n');
    }
    text
}

pub(super) fn draw(app: &mut App, width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal
        .draw(|f| {
            super::super::render(f, app);
        })
        .expect("draw");
    buffer_text(&terminal)
}
