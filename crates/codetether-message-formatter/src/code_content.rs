//! Accumulate lines inside a code fence.
use crate::{MessageFormatter, content_state::ContentState};
pub(crate) fn consume(
    _fmt: &MessageFormatter,
    state: &mut ContentState,
    line: &str,
    _trimmed: &str,
) {
    if state.code_block_start {
        // First line after opening ``` might be language specifier
        state.code_block_start = false;
        if !line.trim().is_empty() && state.code_block_language.is_empty() {
            state.code_block_language = line.trim().to_string();
        } else {
            state.code_block_lines.push(line.to_string());
        }
    } else {
        state.code_block_lines.push(line.to_string());
    }
    return;
}
