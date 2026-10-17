//! Pattern search and occurrence counting.
use super::*;
impl RlmRepl {
    /// Search for lines matching a pattern
    ///
    /// # Arguments
    /// * `pattern` - Regex, or a literal substring if regex parsing fails.
    /// # Returns
    /// Matching one-based line numbers and borrowed line text.
    /// # Examples
    /// ```
    /// use codetether_rlm::repl::{ReplRuntime, RlmRepl};
    /// let mut repl = RlmRepl::new("first\nsecond".into(), ReplRuntime::Rust);
    /// assert_eq!(repl.grep("second")[0].0, 2);
    /// ```
    pub fn grep(&self, pattern: &str) -> Vec<(usize, &str)> {
        let re = match regex::Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => {
                // Fall back to simple contains
                return self
                    .context_lines
                    .iter()
                    .enumerate()
                    .filter(|(_, line)| line.contains(pattern))
                    .map(|(i, line)| (i + 1, line.as_str()))
                    .collect();
            }
        };

        self.context_lines
            .iter()
            .enumerate()
            .filter(|(_, line)| re.is_match(line))
            .map(|(i, line)| (i + 1, line.as_str()))
            .collect()
    }

    /// Count occurrences of a pattern
    ///
    /// # Arguments
    /// * `pattern` - Regex, or literal text if parsing fails.
    /// # Returns
    /// Number of non-overlapping matches.
    /// # Examples
    /// ```
    /// use codetether_rlm::repl::{ReplRuntime, RlmRepl};
    /// let mut repl = RlmRepl::new("first\nsecond".into(), ReplRuntime::Rust);
    /// assert_eq!(repl.count("first"), 1);
    /// ```
    pub fn count(&self, pattern: &str) -> usize {
        let re = match regex::Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => return self.context.matches(pattern).count(),
        };
        re.find_iter(&self.context).count()
    }
}
