//! Bind native request state and token presentation to the shared inspector.
use crate::tui::{app::state::App, token_display::TokenDisplay};
use ratatui::{Frame, layout::Rect};
mod projection;
/// Draw latency information for the current application state.
pub fn render_latency(f: &mut Frame, area: Rect, app: &App) {
    codetether_latency_view::render_latency(f, area, &projection::view(app), || {
        TokenDisplay::new().create_detailed_display()
    });
}
