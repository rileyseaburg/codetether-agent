//! Readable provider reasoning shown before the reply is complete.
use crate::tui::{app::state::AppState, message_formatter::MessageFormatter};
use ratatui::{style::Stylize, text::Line};

pub(super) fn append(
    lines: &mut Vec<Line<'static>>,
    state: &AppState,
    formatter: &MessageFormatter,
) {
    if !state.processing || state.streaming.streaming_reasoning.is_empty() {
        return;
    }
    lines.push(Line::default());
    lines.push(Line::from("Thinking · live".cyan()));
    lines.extend(formatter.format_content(&state.streaming.streaming_reasoning, "assistant"));
}
