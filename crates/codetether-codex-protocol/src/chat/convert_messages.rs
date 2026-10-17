//! convert messages.
use crate::*;
impl CodexProtocol {
    #[allow(dead_code)]
    /// Encode messages in chat-completion format.
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
    /// let encoded = codetether_codex_protocol::CodexProtocol::convert_messages(&[]);
    /// assert!(encoded.is_empty());
    /// ```
    pub fn convert_messages(messages: &[Message]) -> Vec<Value> {
        messages.iter().map(Self::convert_message).collect()
    }
}
