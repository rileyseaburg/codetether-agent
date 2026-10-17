//! Provider extract tool calls.
use crate::*;
impl GeminiWebProvider {
    /// Parse `<tool_call>{...}</tool_call>` JSON blocks from model text.
    ///
    /// Returns:
    /// - cleaned_text: original text with tool-call blocks removed
    /// - calls: parsed `(name, arguments_json_string)` tuples
    #[cfg(test)]
    pub(crate) fn extract_tool_calls(text: &str) -> (String, Vec<(String, String)>) {
        codetether_gemini_web_protocol::GeminiWebProtocol::extract_tool_calls(text)
    }
}
