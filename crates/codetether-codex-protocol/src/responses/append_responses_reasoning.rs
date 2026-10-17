//! append responses reasoning.
use crate::*;
impl CodexProtocol {
    /// Append stored opaque reasoning checkpoint items.
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
    pub fn append_responses_reasoning(message: &Message, input: &mut Vec<Value>) {
        for part in &message.content {
            let ContentPart::Thinking {
                signature: Some(signature),
                ..
            } = part
            else {
                continue;
            };
            if let Some(item) = crate::reasoning::decode_signature(signature) {
                input.push(item);
            }
        }
    }
}
