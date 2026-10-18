//! Native binding for the shared watchdog popup.
use crate::tui::app::state::AppState;
use ratatui::{Frame, layout::Rect};
/// Render the active watchdog notification if present.
pub fn render_watchdog_notification(f: &mut Frame, area: Rect, state: &AppState) {
    codetether_watchdog::render_watchdog_notification(
        f,
        area,
        state.watchdog.notification.as_ref(),
    );
}
