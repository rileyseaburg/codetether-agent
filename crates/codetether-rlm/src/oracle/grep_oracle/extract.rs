//! Extract line-number claims from free-form text.

use super::GrepOracle;
use regex::Regex;
impl GrepOracle {
    /// Try to extract line numbers from free-form text.
    ///
    /// Looks for patterns like "line 42", "L42:", "lines 10-20", etc.
    pub(super) fn extract_line_numbers_from_text(
        &self,
        text: &str,
    ) -> Option<Vec<(usize, String)>> {
        let mut results = Vec::new();

        // Pattern: "line 42: text" or "L42: text" or "42: text"
        let line_re = Regex::new(r"(?i)(?:line\s+|L)?(\d+):\s*(.+)").ok()?;

        for line in text.lines() {
            if let Some(cap) = line_re.captures(line)
                && let (Some(num), Some(content)) = (cap.get(1), cap.get(2))
                && let Ok(line_num) = num.as_str().parse::<usize>()
            {
                results.push((line_num, content.as_str().trim().to_string()));
            }
        }

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }
}
