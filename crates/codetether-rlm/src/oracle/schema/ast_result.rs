//! AstResult wire representation.

use serde::{Deserialize, Serialize};

/// A single AST query result.
///
/// # Examples
/// ```
/// use codetether_rlm::{AstResult};
/// let item = AstResult::function("main".into(), vec![], None, Some((1, 2)));
/// assert_eq!(item.name, "main");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AstResult {
    /// Name of the matched item (function name, struct name, etc.)
    pub name: String,
    /// Function arguments/parameters (as string)
    #[serde(default)]
    pub args: Vec<String>,
    /// Return type (as string)
    #[serde(default)]
    pub return_type: Option<String>,
    /// Span: (start_line, end_line)
    #[serde(default)]
    pub span: Option<(usize, usize)>,
}

impl AstResult {
    /// Create a new AST result for a function.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{AstResult};
    /// let item = AstResult::function("main".into(), vec![], None, Some((1, 2)));
    /// assert_eq!(item.name, "main");
    /// ```
    pub fn function(
        name: String,
        args: Vec<String>,
        return_type: Option<String>,
        span: Option<(usize, usize)>,
    ) -> Self {
        Self {
            name,
            args,
            return_type,
            span,
        }
    }
}
