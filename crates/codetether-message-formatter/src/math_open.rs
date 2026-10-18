//! Open or complete bracket-delimited display math.
use crate::{MessageFormatter, content_state::ContentState};
pub(crate) fn consume(
    fmt: &MessageFormatter,
    state: &mut ContentState,
    _line: &str,
    trimmed: &str,
) {
    let after = &trimmed[2..];
    // Same-line close: \[ x = 1 \]
    if let Some(idx) = after.rfind("\\]") {
        let inner = after[..idx].trim();
        if !inner.is_empty() {
            state.math_block_lines.push(inner.to_string());
        }
        state
            .lines
            .extend(fmt.render_math_block(&state.math_block_lines));
        state.math_block_lines.clear();
        return;
    }
    state.in_math_block = true;
    state.math_delim = "\\[";
    if !after.trim().is_empty() {
        state.math_block_lines.push(after.to_string());
    }
    return;
}
