//! Borrowed access to canonical tool-call fields without allocation.
//!
//! Use [`ContentPart::as_tool_call`] instead of flattening calls into tuples:
//! named fields preserve raw arguments and optional thinking signatures.

use crate::ContentPart;

/// Borrowed view of a [`ContentPart::ToolCall`]'s fields.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{ContentPart, ToolCallRef};
/// let part = ContentPart::ToolCall {
///     id: "c1".into(), name: "example".into(), arguments: "{}".into(),
///     thought_signature: Some("signature".into()),
/// };
/// let call: ToolCallRef<'_> = part.as_tool_call().unwrap();
/// assert_eq!(call.name, "example");
/// assert_eq!(call.thought_signature, Some("signature"));
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
    ///
    /// # Arguments
    ///
    /// * `self` — The content block to inspect.
    ///
    /// # Returns
    ///
    /// A borrowed tool-call view, or `None` for any other content variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_provider_core::ContentPart;
    /// let part = ContentPart::Text { text: "hello".into() };
    /// assert!(part.as_tool_call().is_none());
    /// ```
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
