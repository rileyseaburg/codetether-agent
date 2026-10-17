//! Owned context and variable state for a DSL interpreter.
use super::*;
use std::collections::HashMap;

/// REPL instance for RLM processing
///
/// # Examples
/// ```
/// use codetether_rlm::repl::{ReplRuntime, RlmRepl};
/// let repl = RlmRepl::new("text".into(), ReplRuntime::Rust);
/// assert_eq!(repl.context(), "text");
/// ```
pub struct RlmRepl {
    pub(super) runtime: ReplRuntime,
    pub(super) context: String,
    pub(super) context_lines: Vec<String>,
    pub(super) variables: HashMap<String, String>,
}
impl RlmRepl {
    /// Create a new REPL with the given context
    ///
    /// # Arguments
    /// * `context` - Text to inspect.
    /// * `runtime` - Compatibility runtime label.
    /// # Returns
    /// A REPL with indexed context lines and no variables.
    /// # Examples
    /// ```
    /// use codetether_rlm::repl::{ReplRuntime, RlmRepl};
    /// let mut repl = RlmRepl::new("first\nsecond".into(), ReplRuntime::Rust);
    /// assert_eq!(repl.lines().len(), 2);
    /// ```
    pub fn new(context: String, runtime: ReplRuntime) -> Self {
        let context_lines = context.lines().map(|s| s.to_string()).collect();
        Self {
            runtime,
            context,
            context_lines,
            variables: HashMap::new(),
        }
    }
}
