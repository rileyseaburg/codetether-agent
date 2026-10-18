//! Application adapter for the shared streaming header.
use crate::tui::app::state::AppState;
use ratatui::text::Line;
#[path = "thinking_pulse.rs"]
pub mod thinking_pulse;
/// Render the header and sample elapsed time at its original position.
pub(super) fn streaming_header(
    state: &AppState,
    separator_width: usize,
) -> (Line<'static>, Line<'static>) {
    codetether_chat_status::streaming_header(
        || super::elapsed_badge::elapsed_badge(state),
        separator_width,
    )
}
