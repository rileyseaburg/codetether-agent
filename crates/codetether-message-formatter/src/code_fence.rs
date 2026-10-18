//! Open or close a fenced code block.
use crate::{MessageFormatter, content_state::ContentState};
pub(crate) fn consume(
    fmt: &MessageFormatter,
    state: &mut ContentState,
    line: &str,
    _trimmed: &str,
) {
    if state.in_code_block {
        // End of code block - render with syntax highlighting
        if !state.code_block_lines.is_empty() {
            state
                .lines
                .extend(fmt.render_code_block(&state.code_block_lines, &state.code_block_language));
            state.code_block_lines.clear();
            state.code_block_language.clear();
        }
        state.in_code_block = false;
        state.code_block_start = false;
    } else {
        // Start of code block - extract language
        state.in_code_block = true;
        state.code_block_start = true;
        let lang = line.trim().trim_start_matches('`').trim();
        state.code_block_language = lang.to_string();
    }
    return;
}
