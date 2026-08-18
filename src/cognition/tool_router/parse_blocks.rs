//! `<tool_call>` block extraction from FunctionGemma output.

const OPEN: &str = "<tool_call>";
const CLOSE: &str = "</tool_call>";

/// Yield the trimmed contents of each closed `<tool_call>` block.
///
/// An unclosed opening tag ends iteration, so partial output is ignored rather
/// than misparsed.
pub(super) fn blocks(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut remaining = text;
    while let Some(start) = remaining.find(OPEN) {
        remaining = &remaining[start + OPEN.len()..];
        let Some(end) = remaining.find(CLOSE) else {
            break;
        };
        out.push(remaining[..end].trim());
        remaining = &remaining[end + CLOSE.len()..];
    }
    out
}
