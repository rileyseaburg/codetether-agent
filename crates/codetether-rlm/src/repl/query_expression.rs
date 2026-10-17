//! Evaluate query expression branches in their original order.
use super::*;
impl RlmRepl {
    pub(super) fn query_expression(&mut self, expr: &str) -> Option<String> {
        // head(n)
        if expr.starts_with("head(") {
            let n = self.extract_number(expr).unwrap_or(10);
            return Some(self.head(n).join("\n"));
        }

        // tail(n)
        if expr.starts_with("tail(") {
            let n = self.extract_number(expr).unwrap_or(10);
            return Some(self.tail(n).join("\n"));
        }

        // grep("pattern")
        if expr.starts_with("grep(") {
            let pattern = self.extract_string(expr).unwrap_or_default();
            let matches = self.grep(&pattern);
            return Some(
                matches
                    .iter()
                    .map(|(i, line)| format!("{}:{}", i, line))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        }

        // count("pattern")
        if expr.starts_with("count(") {
            let pattern = self.extract_string(expr).unwrap_or_default();
            return Some(self.count(&pattern).to_string());
        }

        // lines()
        if expr == "lines()" || expr == "lines" {
            return Some(format!("Lines: {}", self.context_lines.len()));
        }

        None
    }
}
