//! Center-panel regressions: cold message cache and minimum size.
//!
//! Bug: the webview read `get_or_build_message_lines`, which returns
//! `None` until the classic chat view warms the cache — so switching to
//! webview on a fresh session showed an empty transcript.

use ratatui::{Terminal, backend::TestBackend};

use super::support::draw;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

#[test]
fn webview_renders_messages_on_cold_cache() {
    let mut app = App::default();
    app.state.messages.push(ChatMessage::new(
        MessageType::User,
        "cold-cache-probe message",
    ));
    let text = draw(&mut app, 100, 30);
    assert!(
        text.contains("cold-cache-probe"),
        "webview center panel must show messages on first render:\n{text}"
    );
}

#[test]
fn webview_reports_too_small_terminal() {
    let mut app = App::default();
    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal
        .draw(|f| assert!(!crate::tui::ui::webview::render(f, &mut app)))
        .expect("draw");
}
