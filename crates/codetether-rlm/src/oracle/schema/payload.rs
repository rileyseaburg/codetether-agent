//! FinalPayload wire representation.

use super::{AstPayload, GrepPayload, SemanticPayload};
use serde::{Deserialize, Serialize};

/// The top-level FINAL() payload envelope.
///
/// # Examples
/// ```
/// use codetether_rlm::{FinalPayload};
/// assert!(matches!(FinalPayload::parse("not JSON"), FinalPayload::Malformed { .. }));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum FinalPayload {
    /// Grep/pattern-match query results
    Grep(GrepPayload),
    /// AST/structural query results
    Ast(AstPayload),
    /// Semantic/free-form text (unverifiable)
    Semantic(SemanticPayload),
    /// Malformed JSON that couldn't be parsed
    Malformed {
        /// The raw string that failed to parse
        raw: String,
        /// Error message from parsing attempt
        error: String,
    },
}
