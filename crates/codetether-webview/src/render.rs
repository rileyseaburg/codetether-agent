//! Ordered Webview panel and overlay composition.
use crate::{Host, chat};
use ratatui::{Frame, widgets::Clear};

/// Render a complete Webview frame.
/// # Arguments
/// * `f` - Destination frame.
/// * `host` - Application panel adapters and current layout inputs.
/// # Returns
/// False after drawing the size hint when the terminal cannot fit Webview;
/// otherwise true after rendering all panels and overlays.
/// # Examples
/// ```no_run
/// fn draw(frame: &mut ratatui::Frame, host: &mut impl codetether_webview::Host) {
///     let rendered = codetether_webview::render(frame, host);
///     if !rendered { /* caller may use its compact layout */ }
/// }
/// ```
pub fn render(f: &mut Frame, host: &mut impl Host) -> bool {
    let area = f.area();
    f.render_widget(Clear, area);
    if chat::terminal_too_small(area) {
        codetether_webview_bars::status::render_too_small(f, area);
        return false;
    }
    let show_inspector = codetether_chat_layout::webview::show_inspector(area);
    let main =
        codetether_chat_layout::webview::compute(area, host.input(), host.suggestions_visible());
    host.header(f, main.header);
    let center_area = if codetether_chat_layout::webview::show_sidebar(area) {
        let body = codetether_chat_layout::webview::webview_body_chunks(main.body, show_inspector);
        host.sidebar(f, body[0]);
        if show_inspector && let Some(area) = body.get(2).copied() {
            host.inspector(f, area);
        }
        body.get(1).copied().unwrap_or(main.body)
    } else {
        main.body
    };
    host.center(f, center_area);
    host.input_panel(f, main.input);
    if let Some(rect) = main.suggestions {
        host.suggestions(f, rect);
    }
    host.status(f, main.status);
    host.approval(f, area);
    host.help(f);
    true
}
