//! Complete pinned source text, independently verifiable against the transcript.
use crate::pages::PageKind;
use codetether_provider_core::{ContentPart, Message, Role};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A pinned source or its explicit supersession, using absolute transcript indices.
/// # Examples
/// ```
/// use codetether_context_policy::{pages::PageKind, state_header::ConstraintRecord};
/// use codetether_provider_core::{ContentPart, Message, Role};
/// let message = Message { role: Role::User,
///     content: vec![ContentPart::Text { text: "Do not deploy".into() }] };
/// let record = ConstraintRecord::from_message(12, &message, PageKind::Constraint).unwrap();
/// assert_eq!(record.text, "Do not deploy");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintRecord {
    /// Absolute source message index, usable with context browsing.
    pub source_turn: usize,
    /// Original speaker; pinning never promotes assistant/tool text to user authority.
    pub source_role: Role,
    /// Complete source text, retaining whitespace and all text parts.
    pub text: String,
    /// SHA-256 of the exact UTF-8 text.
    pub text_hash: String,
    /// Absolute later source explicitly replacing this constraint.
    pub superseded_by: Option<usize>,
}

impl ConstraintRecord {
    /// Check whether the stored digest matches the complete source text.
    /// # Examples
    /// See the construction example on [`ConstraintRecord`].
    pub fn has_valid_hash(&self) -> bool {
        self.text_hash == format!("{:x}", Sha256::digest(self.text.as_bytes()))
    }

    /// Extract a full source record for active or superseded pins.
    /// # Arguments
    /// `source_turn` is absolute; `message` and `page` are the canonical pair.
    /// # Returns
    /// A record for a nonempty pinned source, otherwise `None`.
    pub fn from_message(source_turn: usize, message: &Message, page: PageKind) -> Option<Self> {
        let superseded_by = match page {
            PageKind::Constraint => None,
            PageKind::SupersededConstraint { by_turn } => Some(by_turn),
            _ => return None,
        };
        let text = message
            .content
            .iter()
            .filter_map(|part| match part {
                ContentPart::Text { text } | ContentPart::ToolResult { content: text, .. } => {
                    Some(text.as_str())
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        if text.trim().is_empty() {
            return None;
        }
        Some(Self {
            source_turn,
            source_role: message.role,
            text_hash: format!("{:x}", Sha256::digest(text.as_bytes())),
            text,
            superseded_by,
        })
    }
}
