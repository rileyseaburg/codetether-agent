//! Expand named variables and context-size expressions.
use super::*;
impl RlmRepl {
    pub(super) fn expand_expression(&self, expr: &str) -> String {
        // Simple variable expansion
        let mut result = expr.to_string();

        for (name, value) in &self.variables {
            let patterns = [
                format!("${{{}}}", name),
                format!("${}", name),
                format!("{{{}}}", name),
            ];
            for p in patterns {
                result = result.replace(&p, value);
            }
        }

        // Evaluate embedded expressions
        if result.contains("context.len()") || result.contains("context.length") {
            result = result
                .replace("context.len()", &self.context.len().to_string())
                .replace("context.length", &self.context.len().to_string());
        }

        if result.contains("lines().len()") || result.contains("lines().length") {
            result = result
                .replace("lines().len()", &self.context_lines.len().to_string())
                .replace("lines().length", &self.context_lines.len().to_string());
        }

        result
    }
}
