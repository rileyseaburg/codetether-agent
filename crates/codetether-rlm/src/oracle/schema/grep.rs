//! GrepPayload wire representation.

use super::GrepMatch;
use serde::{Deserialize, Serialize};

/// Grep/pattern-match payload.
///
/// # Examples
/// ```
/// use codetether_rlm::{GrepPayload};
/// let data = GrepPayload { file: "main.rs".into(), pattern: "main".into(), matches: vec![] };
/// assert!(data.matches.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GrepPayload {
    /// File that was searched
    pub file: String,
    /// Regex pattern used
    pub pattern: String,
    /// Matched lines
    pub matches: Vec<GrepMatch>,
}
