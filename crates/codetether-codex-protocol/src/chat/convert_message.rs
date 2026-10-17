//! convert message.
use crate::*;
impl CodexProtocol {
    /// Encode one message according to its role.
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
    pub fn convert_message(message: &Message) -> Value {
        match message.role {
            Role::Tool => Self::convert_tool_message(message),
            Role::Assistant => Self::convert_assistant_message(message),
            Role::System => json!({
                "role": "system",
                "content": Self::message_text(message, "\n"),
            }),
            Role::Developer => json!({
                "role": "developer",
                "content": Self::message_text(message, "\n"),
            }),
            Role::User => json!({
                "role": "user",
                "content": if Self::user_has_images(message) {
                    json!(Self::user_image_content(message, UserImageFormat::Chat))
                } else {
                    json!(Self::message_text(message, "\n"))
                },
            }),
        }
    }
}
