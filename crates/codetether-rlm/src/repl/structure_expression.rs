//! Evaluate structure expression branches in their original order.
use super::*;
impl RlmRepl {
    pub(super) fn structure_expression(&mut self, expr: &str) -> Option<String> {
        // slice(start, end)
        if expr.starts_with("slice(") {
            let nums = self.extract_numbers(expr);
            if nums.len() >= 2 {
                return Some(self.slice(nums[0], nums[1]).to_string());
            }
        }

        // chunks(n)
        if expr.starts_with("chunks(") || expr.starts_with("chunk(") {
            let n = self.extract_number(expr).unwrap_or(5);
            let chunks = self.chunks(n);
            return Some(format!(
                "[{} chunks of {} lines each]",
                chunks.len(),
                chunks.first().map(|c| c.lines().count()).unwrap_or(0)
            ));
        }

        // context
        if expr == "context" || expr.starts_with("context.slice") || expr.starts_with("context[") {
            return Some(format!(
                "[Context: {} chars, {} lines]",
                self.context.len(),
                self.context_lines.len()
            ));
        }

        // ast_query("s-expression")
        // Execute a tree-sitter AST query and return formatted results
        if expr.starts_with("ast_query(") {
            let query = self.extract_string(expr).unwrap_or_default();
            return Some(self.execute_ast_query(&query));
        }

        None
    }
}
