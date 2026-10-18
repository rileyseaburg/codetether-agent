//! Native Webview adapters and regression tests.
pub mod chat;
mod chat_lines;
pub mod header;
pub mod inspector;
pub mod inspector_lines;
pub mod layout;
pub mod layout_mode;
pub mod sessions_panel;
pub mod sidebar;
pub mod status;
pub mod status_prefix;
#[cfg(test)]
mod tests;
pub mod workspace_panel;

mod host;
use crate::tui::app::state::App;
use ratatui::Frame;
/// Render Webview, returning false when the terminal is too small.
pub fn render(f: &mut Frame, app: &mut App) -> bool {
    codetether_webview::render(f, &mut host::Native(app))
}
