//! Reviewed replacements; application belongs to the editor, not this backend.
use serde::{Deserialize, Serialize};

/// One exact replacement, matched once in the snapshot.
/// # Examples
/// ```
/// let e = codetether_rust_repair::Edit { path: "src/lib.rs".into(), old_text: "bad".into(), new_text: "good".into() };
/// assert_ne!(e.old_text, e.new_text);
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edit {
    /// Explicitly editable relative snapshot path.
    pub path: String,
    /// Nonempty exact-match text, occurring once.
    pub old_text: String,
    /// Different replacement text.
    pub new_text: String,
}

/// A repair proposal, or zero edits with an explanation when no fix is safe.
/// # Examples
/// ```
/// let p = codetether_rust_repair::Proposal { schema_version: 1, summary: "No safe repair".into(), edits: vec![] };
/// assert!(p.edits.is_empty());
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Proposal {
    /// Wire schema, currently exactly 1.
    pub schema_version: u32,
    /// Explanation of the edits or why none are safe.
    pub summary: String,
    /// At most eight edits with unique paths.
    pub edits: Vec<Edit>,
}
