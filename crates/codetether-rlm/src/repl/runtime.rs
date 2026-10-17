//! Runtime selection retained by the compatibility interpreter.
use serde::{Deserialize, Serialize};

/// REPL runtime options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
/// Runtime labels: Rust, Bun, and Python; in-memory execution uses the DSL.
///
/// # Examples
/// ```
/// use codetether_rlm::repl::ReplRuntime;
/// let label = match ReplRuntime::default() {
///     ReplRuntime::Rust => "rust",
///     ReplRuntime::Bun => "bun",
///     ReplRuntime::Python => "python",
/// };
/// assert_eq!(label, "rust");
/// ```
pub enum ReplRuntime {
    /// In-memory DSL interpreter
    #[default]
    Rust,
    /// Bun/Node.js JavaScript REPL
    Bun,
    /// Python REPL
    Python,
}
