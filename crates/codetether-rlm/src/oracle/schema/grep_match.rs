//! GrepMatch wire representation.

use serde::{Deserialize, Serialize};

/// A single grep match with line number and text.
///
/// # Examples
/// ```
/// use codetether_rlm::{GrepMatch};
/// let line = GrepMatch::new(1, "fn main() {}".into());
/// assert_eq!(line.line, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GrepMatch {
    /// Line number (1-indexed, matching `grep -n`)
    pub line: usize,
    /// Full text of the matched line (or substring)
    pub text: String,
}

impl GrepMatch {
    /// Create a new match.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{GrepMatch};
    /// let line = GrepMatch::new(1, "fn main() {}".into());
    /// assert_eq!(line.line, 1);
    /// ```
    pub fn new(line: usize, text: String) -> Self {
        Self { line, text }
    }

    /// Check if this match's text is a substring of the actual line.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{GrepMatch};
    /// assert!(GrepMatch::new(1, "main".into()).text_matches("fn main() {}"));
    /// ```
    pub fn text_matches(&self, actual_line: &str) -> bool {
        actual_line.contains(&self.text)
    }
}
