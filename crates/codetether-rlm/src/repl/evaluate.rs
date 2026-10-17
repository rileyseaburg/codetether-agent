//! Evaluate query helpers before variable and literal fallback.
use super::*;
impl RlmRepl {
    /// Evaluate a query, variable, or literal expression.
    ///
    /// # Arguments
    /// * `expr` - Query, variable, or literal expression.
    /// # Returns
    /// The expression value as text, preserving unrecognized expressions.
    /// # Examples
    /// ```
    /// use codetether_rlm::repl::{ReplRuntime, RlmRepl};
    /// let mut repl = RlmRepl::new("first\nsecond".into(), ReplRuntime::Rust);
    /// assert_eq!(repl.evaluate_expression("head(1)"), "first");
    /// ```
    /// # Panics
    /// Panics for an empty context chunked with a positive count; malformed
    /// argument boundaries may also panic during expression parsing.
    pub fn evaluate_expression(&mut self, expr: &str) -> String {
        let expr = expr.trim().trim_end_matches(';');

        if let Some(value) = self.query_expression(expr) {
            return value;
        }
        if let Some(value) = self.structure_expression(expr) {
            return value;
        }
        // Variable reference
        if let Some(val) = self.get_var(expr) {
            return val.to_string();
        }

        // String literal
        if (expr.starts_with('"') && expr.ends_with('"'))
            || (expr.starts_with('\'') && expr.ends_with('\''))
        {
            let mut chars = expr.chars();
            let _ = chars.next();
            let _ = chars.next_back();
            return chars.collect();
        }

        expr.to_string()
    }
}
