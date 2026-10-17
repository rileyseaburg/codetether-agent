//! Grep source storage and line matching.

use anyhow::Result;
use regex::Regex;

/// Grep-based oracle for validating pattern-match queries.
///
/// # Examples
/// ```
/// use codetether_rlm::{GrepOracle};
/// let oracle = GrepOracle::new("fn main() {}".into());
/// assert_eq!(oracle.grep("main").unwrap().len(), 1);
/// ```
pub struct GrepOracle {
    /// Source code content
    #[allow(dead_code)]
    pub(super) source: String,
    /// Source code as lines (1-indexed when displayed)
    pub(super) source_lines: Vec<String>,
}
impl GrepOracle {
    /// Create a new grep oracle for the given source file content.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{GrepOracle};
    /// let oracle = GrepOracle::new("fn main() {}".into());
    /// assert_eq!(oracle.grep("main").unwrap().len(), 1);
    /// ```
    pub fn new(source: String) -> Self {
        let source_lines = source.lines().map(|s| s.to_string()).collect();
        Self {
            source,
            source_lines,
        }
    }

    /// Run grep on the source and return matches with line numbers.
    ///
    /// # Errors
    /// Returns an error when the pattern is not a valid regular expression.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{GrepOracle};
    /// let oracle = GrepOracle::new("fn main() {}".into());
    /// assert_eq!(oracle.grep("main").unwrap()[0].0, 1);
    /// assert!(oracle.grep("[").is_err());
    /// ```
    pub fn grep(&self, pattern: &str) -> Result<Vec<(usize, String)>> {
        let re = Regex::new(pattern)?;

        let matches: Vec<(usize, String)> = self
            .source_lines
            .iter()
            .enumerate()
            .filter(|(_, line)| re.is_match(line))
            .map(|(i, line)| (i + 1, line.clone())) // 1-indexed
            .collect();

        Ok(matches)
    }
}
