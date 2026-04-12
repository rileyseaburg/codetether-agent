//! Coarse query classification used by the validator to pick an
//! oracle backend.
//!
//! Each variant maps to a different verification strategy — grep,
//! tree-sitter AST, or "none" for semantic questions that cannot
//! be deterministically checked.
//!
//! # Examples
//!
//! ```ignore
//! match classify(&query) {
//!     QueryType::PatternMatch => run_grep_oracle(),
//!     QueryType::Structural  => run_tree_sitter_oracle(),
//!     QueryType::Semantic    => skip_verification(),
//! }
//! ```

/// Describes the kind of evidence an oracle needs to verify a
/// FINAL() answer.
///
/// The validator inspects the query text and the `FinalPayload`
/// to choose the right variant before dispatching.
///
/// # Examples
///
/// ```ignore
/// let qt = QueryType::PatternMatch;
/// assert_eq!(qt, QueryType::PatternMatch);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    /// Line-oriented text matching verified by the grep oracle.
    ///
    /// Covers queries like "find all async functions" that produce
    /// line-numbered output.
    PatternMatch,
    /// AST-level structural checks verified by tree-sitter.
    ///
    /// Covers queries about function signatures, struct fields,
    /// and other syntactic constructs.
    Structural,
    /// Free-form reasoning that has no deterministic oracle.
    ///
    /// Traces with this classification can only reach `Unverified`
    /// status, never `Golden`.
    Semantic,
}
