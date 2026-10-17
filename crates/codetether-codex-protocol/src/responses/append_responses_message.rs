//! append responses message.
use crate::*;
impl CodexProtocol {
    /// Append Responses items for one message role.
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
    pub fn append_responses_message(
        message: &Message,
        input: &mut Vec<Value>,
        known_calls: &mut std::collections::HashSet<String>,
    ) {
        match message.role {
            Role::System => {}
            Role::Developer => input.push(Self::convert_message(message)),
            Role::User => Self::append_responses_user(message, input),
            Role::Assistant => Self::append_responses_assistant(message, input, known_calls),
            Role::Tool => Self::append_responses_tool(message, input, known_calls),
        }
    }
}
