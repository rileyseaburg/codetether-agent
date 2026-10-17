//! Parse numeric and string arguments from DSL expressions.
use super::*;
impl RlmRepl {
    pub(super) fn extract_number(&self, expr: &str) -> Option<usize> {
        let start = expr.find('(')?;
        let end = expr.find(')')?;
        let inner = expr[start + 1..end].trim();
        inner.parse().ok()
    }

    pub(super) fn extract_numbers(&self, expr: &str) -> Vec<usize> {
        let start = expr.find('(').unwrap_or(0);
        let end = expr.find(')').unwrap_or(expr.len());
        let inner = &expr[start + 1..end];

        inner
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    }

    pub(super) fn extract_string(&self, expr: &str) -> Option<String> {
        let start = expr.find('(')?;
        let end = expr.rfind(')')?;
        let inner = expr[start + 1..end].trim();

        // Remove quotes
        let unquoted = inner
            .trim_start_matches(['"', '\'', '`', '/'])
            .trim_end_matches(['"', '\'', '`', '/']);

        Some(unquoted.to_string())
    }
}
