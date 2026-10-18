//! Application bindings for the shared help view.
use crate::tui::app::state::AppState;
pub use codetether_help_view::{HelpScrollState, centered_rect};
use ratatui::{Frame, text::Line};
pub fn build_help_lines(app_state: &AppState) -> Vec<Line<'static>> {
    let context = codetether_help_view::HelpContext {
        session_id: &app_state.session_id,
        cwd_display: &app_state.cwd_display,
        auto_apply_edits: app_state.auto_apply_edits,
        allow_network: app_state.allow_network,
        slash_autocomplete: app_state.slash_autocomplete,
        worker_id: &app_state.worker_id,
        worker_name: &app_state.worker_name,
        a2a_connected: app_state.a2a_connected,
    };
    codetether_help_view::build_help_lines(&context)
}
pub fn render_help_overlay_if_needed(f: &mut Frame, app_state: &mut AppState) {
    if !app_state.show_help {
        return;
    }
    let area = centered_rect(60, 60, f.area());
    let lines = build_help_lines(app_state);
    codetether_help_view::render_help_overlay(f, area, lines, &mut app_state.help_scroll);
}
