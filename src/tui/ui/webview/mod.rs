pub mod chat;
mod chat_lines;
pub mod header;
pub mod inspector;
pub mod inspector_lines;
pub mod layout;
pub mod layout_mode;
mod main_chunks;
pub mod sessions_panel;
pub mod sidebar;
pub mod status;
pub mod status_prefix;
#[cfg(test)]
mod tests;
pub mod workspace_panel;

use ratatui::Frame;

use crate::tui::app::state::App;

/// Render the full webview chat layout. Returns `false` if terminal too small.
pub fn render(f: &mut Frame, app: &mut App) -> bool {
    let area = f.area();
    if chat::terminal_too_small(area) {
        status::render_too_small(f, area);
        return false;
    }
    let show_inspector = layout::show_inspector(area);
    let main = main_chunks::compute(area, app);
    header::render_webview_header(f, app, main.header);
    let body = layout::webview_body_chunks(main.body, show_inspector);
    sidebar::render_webview_sidebar(f, app, body[0]);
    let center_area = body.get(1).copied().unwrap_or(main.body);
    if show_inspector && let Some(area) = body.get(2).copied() {
        inspector::render_webview_inspector(f, app, area);
    }
    chat_lines::render_center(f, app, center_area);
    chat::render_webview_input(f, app, main.input);
    if let Some(rect) = main.suggestions {
        crate::tui::ui::chat_view::suggestions::render_suggestions(f, app, rect);
    }
    status::render_webview_status(f, app, main.status);
    crate::tui::ui::chat_view::approval_overlay::render(f, area);
    crate::tui::help::render_help_overlay_if_needed(f, &mut app.state);
    true
}
