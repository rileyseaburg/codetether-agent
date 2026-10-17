//! convert messages to responses input.
use crate::*;

impl CodexProtocol {
    /// Build Responses input with pairing repair and a nonempty anchor.
    ///
    /// # Arguments
    /// Accepts provider-neutral message/tool content or the mutable request
    /// projection and known call identifiers used during conversion.
    ///
    /// # Returns
    /// The encoded value or, for append methods, updates the supplied projection.
    ///
    /// # Examples
    /// ```
    /// let encoded = codetether_codex_protocol::CodexProtocol::convert_messages_to_responses_input(&[]);
    /// assert!(!encoded.is_empty());
    /// ```
    pub fn convert_messages_to_responses_input(messages: &[Message]) -> Vec<Value> {
        let mut input = Vec::new();
        let mut known_calls = std::collections::HashSet::new();
        for message in messages {
            Self::append_responses_message(message, &mut input, &mut known_calls);
        }
        crate::request_anchor::ensure(crate::pairing::repair(input))
    }
}
