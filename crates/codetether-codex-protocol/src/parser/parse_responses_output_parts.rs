//! parse responses output parts.
use crate::*;
impl CodexProtocol {
    #[allow(dead_code)]
    /// Convert buffered text and tool output, reporting whether tools occurred.
    pub fn parse_responses_output_parts(output: &[Value]) -> (Vec<ContentPart>, bool) {
        let mut parts = Vec::new();
        let mut has_tool_calls = false;
        for item in output {
            match item.get("type").and_then(Value::as_str) {
                Some("message") => Self::append_response_output_text(item, &mut parts),
                Some("function_call") => {
                    has_tool_calls |= Self::append_response_output_tool(item, &mut parts);
                }
                _ => {}
            }
        }
        (parts, has_tool_calls)
    }
}
