use crate::{ContentPart, Role};
use serde::{Deserialize, Serialize};

/// A message in a conversation, with ordered content blocks.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{Message, Role, ContentPart};
/// let message = Message {
///     role: Role::User,
///     content: vec![ContentPart::Text { text: "hello".into() }],
/// };
/// assert_eq!(message.role, Role::User);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Who sent this message.
    pub role: Role,
    /// Ordered content blocks (text, images, tool calls, etc.).
    pub content: Vec<ContentPart>,
}
