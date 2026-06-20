//! Borrowed accessor for the canonical [`ContentPart::ToolCall`] protocol.
//!
//! Downstream code (agent loop, session, a2a, swarm) should consume tool
//! calls through [`ContentPart::as_tool_call`] instead of flattening them
//! into positional tuples — tuples erase field meaning and silently drop
//! `thought_signature`, which thinking models require across turns.

use super::ContentPart;

/// Borrowed view of a [`ContentPart::ToolCall`]'s fields.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::ContentPart;
/// let part = ContentPart::ToolCall {
///     id: "c1".into(),
///     name: "bash".into(),
///     arguments: "{}".into(),
///     thought_signature: None,
/// };
/// let call = part.as_tool_call().unwrap();
/// assert_eq!(call.name, "bash");
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ToolCallRef<'a> {
    /// Provider-assigned call id.
    pub id: &'a str,
    /// Tool (function) name.
    pub name: &'a str,
    /// Raw JSON-encoded arguments (source of truth; parse locally if needed).
    pub arguments: &'a str,
    /// Thinking-model signature, if present.
    pub thought_signature: Option<&'a str>,
}

impl ContentPart {
    /// Returns borrowed fields when this part is a [`ContentPart::ToolCall`].
    pub fn as_tool_call(&self) -> Option<ToolCallRef<'_>> {
        match self {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                thought_signature,
            } => Some(ToolCallRef {
                id,
                name,
                arguments,
                thought_signature: thought_signature.as_deref(),
            }),
            _ => None,
        }
    }
}
