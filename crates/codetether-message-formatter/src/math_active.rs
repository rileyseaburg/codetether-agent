//! Continue or close a display-math block.
use crate::{MessageFormatter, content_state::ContentState};
pub(crate) fn consume(fmt: &MessageFormatter, state: &mut ContentState, line: &str, trimmed: &str) {
    let close = if state.math_delim == "\\[" {
        "\\]"
    } else {
        "$$"
    };
    if state.math_delim == "\\[" && trimmed.ends_with(close) {
        let before = trimmed.trim_end_matches(close);
        if !before.trim().is_empty() {
            state.math_block_lines.push(before.to_string());
        }
        state
            .lines
            .extend(fmt.render_math_block(&state.math_block_lines));
        state.math_block_lines.clear();
        state.in_math_block = false;
        return;
    }
    if state.math_delim == "$$" && trimmed == "$$" {
        state
            .lines
            .extend(fmt.render_math_block(&state.math_block_lines));
        state.math_block_lines.clear();
        state.in_math_block = false;
        return;
    }
    state.math_block_lines.push(line.to_string());
    return;
}
