//! Exact source spans replaced by later, independently retained user sources.
use super::ConstraintRecord;
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

/// An immutable link from one source span to a later user correction.
/// Byte offsets address the complete UTF-8 text identified by `source_hash`.
/// # Examples
/// ```
/// use codetether_context_policy::state_header::ConstraintCorrection;
/// let link: ConstraintCorrection = serde_json::from_value(serde_json::json!({
///     "source_turn":0,"source_hash":"hash","start":0,"end":4,
///     "replacement_turn":2,"replacement_hash":"new hash"
/// })).unwrap();
/// assert_eq!(link.end, 4);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintCorrection {
    /// Absolute original source turn.
    pub source_turn: usize,
    /// SHA-256 of its complete text.
    pub source_hash: String,
    /// Inclusive UTF-8 byte offset of the replaced text.
    pub start: usize,
    /// Exclusive UTF-8 byte offset of the replaced text.
    pub end: usize,
    /// Absolute later user source explaining the replacement.
    pub replacement_turn: usize,
    /// SHA-256 of the complete replacement source.
    pub replacement_hash: String,
}
impl ConstraintCorrection {
    /// Validate identity, ordering, roles and UTF-8 boundaries against sources.
    /// # Arguments
    /// `source` and `replacement` are complete canonical pinned records.
    /// # Returns
    /// Unit when the link still identifies the original user evidence.
    /// # Errors
    /// Returns an error for altered, superseded, non-user or invalid spans.
    /// # Examples
    /// ```
    /// use codetether_context_policy::{pages::PageKind, state_header::{ConstraintCorrection, ConstraintRecord}};
    /// use codetether_provider_core::{ContentPart, Message, Role};
    /// let record = |turn, text: &str| ConstraintRecord::from_message(turn,
    ///     &Message { role: Role::User, content: vec![ContentPart::Text { text: text.into() }] },
    ///     PageKind::Constraint).unwrap();
    /// let old = record(0, "red");
    /// let new = record(1, "blue");
    /// let link = ConstraintCorrection { source_turn: 0, source_hash: old.text_hash.clone(),
    ///     start: 0, end: 3, replacement_turn: 1, replacement_hash: new.text_hash.clone() };
    /// assert!(link.validate(&old, &new).is_ok());
    /// ```
    pub fn validate(
        &self,
        source: &ConstraintRecord,
        replacement: &ConstraintRecord,
    ) -> Result<()> {
        ensure!(
            source.source_turn == self.source_turn
                && replacement.source_turn == self.replacement_turn
                && self.replacement_turn > self.source_turn,
            "correction source order or identity changed"
        );
        ensure!(
            source.has_valid_hash()
                && replacement.has_valid_hash()
                && source.text_hash == self.source_hash
                && replacement.text_hash == self.replacement_hash,
            "correction source text changed"
        );
        ensure!(
            source.source_role == codetether_provider_core::Role::User
                && replacement.source_role == codetether_provider_core::Role::User,
            "corrections require user-authored sources"
        );
        ensure!(
            source.superseded_by.is_none(),
            "original correction source was wholly superseded"
        );
        ensure!(
            self.start < self.end && source.text.get(self.start..self.end).is_some(),
            "correction span is outside UTF-8 source boundaries"
        );
        Ok(())
    }
}
