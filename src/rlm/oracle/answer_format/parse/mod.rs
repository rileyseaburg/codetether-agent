//! Parsing logic for [`FinalAnswerFormat`](super::FinalAnswerFormat).
//!
//! Delegates to per-format strategies in [`strategies`] and
//! exposes [`extract_count_from_text`] for reuse.
//!
//! # Examples
//!
//! ```ignore
//! let fmt = FinalAnswerFormat::parse("Found 15 matches");
//! ```

mod strategies;

use strategies::{try_parse_count, try_parse_json, try_parse_line_numbered};

use super::FinalAnswerFormat;

impl FinalAnswerFormat {
    /// Parse a FINAL() answer string into its classified format.
    ///
    /// Tries each format in priority order: line-numbered →
    /// count → JSON → free-form text.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let fmt = FinalAnswerFormat::parse("Found 15 matches");
    /// assert!(matches!(fmt, FinalAnswerFormat::CountResult { .. }));
    /// ```
    pub fn parse(answer: &str) -> Self {
        if let Some(fmt) = try_parse_line_numbered(answer) {
            return fmt;
        }
        if let Some(fmt) = try_parse_count(answer) {
            return fmt;
        }
        if let Some(fmt) = try_parse_json(answer) {
            return fmt;
        }
        Self::FreeFormText {
            text: answer.to_string(),
        }
    }
}

/// Extract a count number from natural language text
/// like "Found 15 async functions" or "count: 42".
///
/// Returns `None` when no recognizable numeric pattern is found.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(extract_count_from_text("Found 15 items"), Some(15));
/// assert_eq!(extract_count_from_text("no numbers here"), None);
/// ```
pub fn extract_count_from_text(text: &str) -> Option<usize> {
    use std::sync::LazyLock;
    static RE: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(
            r"(?i)(?:found|count:?\s*)\s*(\d+)|(\d+)\s+(?:functions?|matches?|occurrences?|items?|results?)"
        ).expect("invalid regex")
    });
    for cap in RE.captures_iter(text) {
        if let Some(m) = cap.get(1) {
            if let Ok(n) = m.as_str().parse() {
                return Some(n);
            }
        }
        if let Some(m) = cap.get(2) {
            if let Ok(n) = m.as_str().parse() {
                return Some(n);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests;
