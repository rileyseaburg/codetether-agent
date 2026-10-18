//! Handle prose, blank lines and candidate table rows.
use crate::{MessageFormatter, content_state::ContentState};
use crate::{block as message_formatter_block, table as message_formatter_table};
use ratatui::text::Line;
pub(crate) fn consume(fmt: &MessageFormatter, state: &mut ContentState, line: &str, role: &str) {
    // Handle regular text with enhanced formatting
    if line.trim().is_empty() {
        message_formatter_table::flush(fmt, &mut state.table_buf, &mut state.lines, role);
        state.lines.push(Line::from(""));
        return;
    }

    // Buffer consecutive table-like rows; render as a box on flush.
    if message_formatter_table::is_table_row(line) {
        state.table_buf.push(line.to_string());
        return;
    }
    message_formatter_table::flush(fmt, &mut state.table_buf, &mut state.lines, role);

    let formatted_line = message_formatter_block::format_line(fmt, line, role);
    state
        .lines
        .extend(fmt.wrap_line(formatted_line, fmt.max_width.saturating_sub(4)));
}
