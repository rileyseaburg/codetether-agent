//! convert responses tools.
use crate::*;
impl CodexProtocol {
    /// Encode Responses tool definitions.
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
    pub fn convert_responses_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "name": t.name,
                    "description": t.description,
                    "strict": false,
                    "parameters": t.parameters
                })
            })
            .collect()
    }
}
