//! SemanticPayload wire representation.

use serde::{Deserialize, Serialize};

/// Semantic/free-form text payload (unverifiable).
///
/// # Examples
/// ```
/// use codetether_rlm::{SemanticPayload};
/// let item = SemanticPayload::new("main.rs".into(), "explanation".into());
/// assert_eq!(item.file, "main.rs");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticPayload {
    /// File that was analyzed
    pub file: String,
    /// Free-form text answer
    pub answer: String,
}

impl SemanticPayload {
    /// Create a new semantic payload.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{SemanticPayload};
    /// let item = SemanticPayload::new("main.rs".into(), "explanation".into());
    /// assert_eq!(item.file, "main.rs");
    /// ```
    pub fn new(file: String, answer: String) -> Self {
        Self { file, answer }
    }
}
