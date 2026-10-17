//! Resolve explicit literals before the ordered pattern catalog.

use super::GrepOracle;
use regex::Regex;
impl GrepOracle {
    /// Infer a grep pattern from the query string.
    ///
    /// Returns None if the pattern cannot be reliably inferred.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{GrepOracle};
    /// assert_eq!(GrepOracle::infer_pattern("grep for 'main'"), Some("main".into()));
    /// ```
    pub fn infer_pattern(query: &str) -> Option<String> {
        let lower = query.to_lowercase();

        if let Some(pattern) = super::literal::infer(query) {
            return Some(pattern);
        }
        for (pattern_re, grep_pattern) in super::patterns::PATTERNS {
            if let Ok(re) = Regex::new(pattern_re)
                && re.is_match(&lower)
            {
                return Some(grep_pattern.to_string());
            }
        }

        None
    }
}
