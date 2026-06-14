//! Overlay regressions: slash suggestions and streaming preview, which
//! the webview previously dropped entirely.

use super::support::draw;
use crate::tui::app::state::App;

#[test]
fn webview_shows_slash_suggestions() {
    let mut app = App::default();
    app.state.input = "/he".to_string();
    app.state.refresh_slash_suggestions();
    assert!(app.state.slash_suggestions_visible());
    let text = draw(&mut app, 100, 30);
    assert!(
        text.contains("/help"),
        "slash autocomplete panel must render in webview:\n{text}"
    );
}

#[test]
fn webview_shows_streaming_preview() {
    let mut app = App::default();
    app.state.processing = true;
    app.state.streaming_text = "streaming-probe tokens".to_string();
    let text = draw(&mut app, 100, 30);
    assert!(
        text.contains("streaming-probe"),
        "streaming preview must render in webview:\n{text}"
    );
}
