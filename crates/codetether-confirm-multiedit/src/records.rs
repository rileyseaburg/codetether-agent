//! Requested replacements and their serialized preview records.
use serde::{Deserialize, Serialize};

/// One requested exact replacement in a named file.
///
/// # Examples
/// ```rust
/// let op = codetether_confirm_multiedit::EditOperation {
///     file: "file.txt".into(), old_string: "old".into(), new_string: "new".into(),
/// };
/// assert_eq!(op.file, "file.txt");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditOperation {
    /// File path supplied by the caller.
    pub file: String,
    /// Original text that must match exactly once.
    pub old_string: String,
    /// Replacement text.
    pub new_string: String,
}

/// Colored diff and line counts for one file replacement.
///
/// # Examples
/// ```rust
/// let preview = codetether_confirm_multiedit::EditPreview {
///     file: "file.txt".into(), diff: "-old\n+new".into(), added: 1, removed: 1,
/// };
/// assert_eq!(preview.added, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditPreview {
    /// File path supplied by the caller.
    pub file: String,
    /// ANSI-colored line diff.
    pub diff: String,
    /// Number of inserted lines.
    pub added: usize,
    /// Number of deleted lines.
    pub removed: usize,
}
