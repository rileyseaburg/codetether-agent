//! Application adapter for shared streaming preview rendering.
use crate::tui::{app::state::AppState, message_formatter::MessageFormatter};
pub use codetether_message_cache::reset_stream_parse_cache;
use ratatui::text::Line;
mod events;
mod reasoning;
/// Append the active stream, sampling elapsed time inside the shared header.
///
/// # Arguments
/// * `lines` - Destination transcript rows.
/// * `state` - Current stream text and timing state.
/// * `separator_width` - Separator width.
/// * `formatter` - Content formatter.
///
/// # Returns
/// Appends visible streaming content while the turn is active.
///
/// # Examples
/// ```no_run
/// # fn example(state: &codetether_agent::tui::app::state::AppState) {
/// let formatter = codetether_agent::tui::message_formatter::MessageFormatter::new(80);
/// let mut lines = vec![];
/// codetether_agent::tui::ui::chat_view::streaming::push_streaming_preview(&mut lines, state, 60, &formatter);
/// # }
/// ```
pub fn push_streaming_preview(
    lines: &mut Vec<Line<'static>>,
    state: &AppState,
    separator_width: usize,
    formatter: &MessageFormatter,
) {
    reasoning::append(lines, state, formatter);
    codetether_tool_panel::push_streaming_preview(
        lines,
        state.processing,
        &state.streaming.streaming_text,
        separator_width,
        formatter,
        || state.request_timing.current_request_elapsed_ms(),
    );
    events::append(lines, state, separator_width);
}
