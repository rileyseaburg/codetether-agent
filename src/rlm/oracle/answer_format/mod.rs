//! Structured representation of an RLM FINAL() answer.
//!
//! The oracle validator parses the raw answer string into one of
//! these variants so it can choose the right verification strategy
//! (grep diff, AST comparison, or "skip").
//!
//! # Examples
//!
//! ```ignore
//! let fmt = FinalAnswerFormat::parse("42:async fn foo()\n100:pub struct Bar");
//! match fmt {
//!     FinalAnswerFormat::LineNumberedMatches { matches } => { /* verify */ }
//!     _ => { /* skip */ }
//! }
//! ```

mod parse;

pub use parse::extract_count_from_text;

/// A parsed FINAL() answer, classified by its structure.
///
/// Constructed via [`FinalAnswerFormat::parse`] which tries each
/// format in priority order: line-numbered → count → JSON → text.
///
/// # Examples
///
/// ```ignore
/// let fmt = FinalAnswerFormat::parse("Found 15 async functions");
/// assert!(matches!(fmt, FinalAnswerFormat::CountResult { count: 15 }));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum FinalAnswerFormat {
    /// Zero or more `line_number:content` pairs extracted from
    /// the answer text (e.g., `42:async fn foo()`).
    LineNumberedMatches { matches: Vec<(usize, String)> },
    /// A single integer extracted from prose like
    /// "Found 15 occurrences".
    CountResult { count: usize },
    /// A successfully-parsed JSON value (object or array).
    StructuredData { data: serde_json::Value },
    /// Anything that doesn't match the above formats.
    ///
    /// Semantic answers fall here and cannot be deterministically
    /// verified.
    FreeFormText { text: String },
}
