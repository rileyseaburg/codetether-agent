//! Structured representation of a FINAL answer.

mod parse;

/// Classification of an RLM FINAL() answer format.
///
/// # Examples
/// ```
/// use codetether_rlm::oracle::FinalAnswerFormat;
/// let value = FinalAnswerFormat::parse("1:fn main() {}");
/// assert!(matches!(value,FinalAnswerFormat::LineNumberedMatches{..}));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum FinalAnswerFormat {
    /// Line-numbered matches (e.g., "42:async fn foo()", "100:pub struct Bar")
    LineNumberedMatches { matches: Vec<(usize, String)> },
    /// Count result (e.g., "Found 15 occurrences")
    CountResult { count: usize },
    /// Structured data (e.g., function signature JSON)
    StructuredData { data: serde_json::Value },
    /// Free-form text (semantic - no deterministic verification)
    FreeFormText { text: String },
}
