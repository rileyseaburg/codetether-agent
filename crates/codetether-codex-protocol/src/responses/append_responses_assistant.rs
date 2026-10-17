//! append responses assistant.
use crate::*;
impl CodexProtocol {
    /// Append assistant reasoning, text, and function-call items.
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
    pub fn append_responses_assistant(
        message: &Message,
        input: &mut Vec<Value>,
        known_calls: &mut std::collections::HashSet<String>,
    ) {
        Self::append_responses_reasoning(message, input);
        let text = Self::message_text(message, "");
        if !text.is_empty() {
            input.push(json!({
                "type": "message",
                "role": "assistant",
                "content": [{ "type": "output_text", "text": text }],
            }));
        }
        for part in &message.content {
            if let ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } = part
            {
                known_calls.insert(id.clone());
                input.push(json!({
                    "type": "function_call",
                    "call_id": id,
                    "name": name,
                    "arguments": arguments,
                }));
            }
        }
    }
}
