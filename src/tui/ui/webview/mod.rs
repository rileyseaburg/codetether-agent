pub mod chat;
pub mod header;
pub mod layout;
pub mod layout_mode;
pub mod sidebar;
pub mod status;

use ratatui::Frame;

use crate::tui::app::state::App;

/// Render the full webview chat layout. Returns `false` if terminal too small.
pub fn render(f: &mut Frame, app: &mut App) -> bool {
    let area = f.area();
    if chat::terminal_too_small(area) {
        status::render_too_small(f, area);
        return false;
    }
    let show_inspector = false;
    let main = layout::webview_main_chunks(area);
    header::render_webview_header(f, app, main[0]);
    let body = layout::webview_body_chunks(main[1], show_inspector);
    sidebar::render_webview_sidebar(f, app, body[0]);
    let center_area = body.get(1).copied().unwrap_or(main[1]);
    let max_w = center_area.width.saturating_sub(4) as usize;
    let lines = app
        .state
        .get_or_build_message_lines(max_w)
        .unwrap_or_default();
    let vis = center_area.height.saturating_sub(2) as usize;
    app.state.chat_last_max_scroll = lines.len().saturating_sub(vis);
    chat::render_webview_chat_center(f, app, center_area, &lines);
    chat::render_webview_input(f, app, main[2]);
    status::render_webview_status(f, app, main[3]);
    true
}
