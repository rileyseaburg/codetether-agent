//! convert tool message.
use crate::*;
impl CodexProtocol {
    /// Encode the first tool result of a tool message.
    ///
    /// # Arguments
    /// Accepts provider-neutral message/tool content or the mutable request
    /// projection and known call identifiers used during conversion.
    ///
    /// # Returns
    /// The encoded value or, for append methods, updates the supplied projection.
    ///
    /// # Examples
    /// See [CodexProtocol::convert_messages_to_responses_input] for the request encoding entry point.
    pub fn convert_tool_message(message: &Message) -> Value {
        let Some(ContentPart::ToolResult {
            tool_call_id,
            content,
        }) = message.content.first()
        else {
            return json!({ "role": "tool", "content": "" });
        };
        json!({
            "role": "tool",
            "tool_call_id": tool_call_id,
            "content": content,
        })
    }
}
