//! AstPayload wire representation.

use super::AstResult;
use serde::{Deserialize, Serialize};

/// AST/structural query payload.
///
/// # Examples
/// ```
/// use codetether_rlm::{AstPayload};
/// let data = AstPayload { file: "main.rs".into(), query: "functions".into(), results: vec![] };
/// assert!(data.results.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstPayload {
    /// File that was queried
    pub file: String,
    /// Tree-sitter query or query type
    pub query: String,
    /// Query results
    pub results: Vec<AstResult>,
}
