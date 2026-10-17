//! extract tool calls.
use crate::*;
impl GeminiWebProtocol {
    /// Parse `<tool_call>{...}</tool_call>` JSON blocks from model text.
    ///
    /// Returns:
    /// - cleaned_text: original text with tool-call blocks removed
    /// - calls: parsed `(name, arguments_json_string)` tuples
    pub fn extract_tool_calls(text: &str) -> (String, Vec<(String, String)>) {
        tool_calls::extract(text)
    }
}
