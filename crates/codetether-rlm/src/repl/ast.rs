//! Render tree-sitter query results against the stored context.
use super::*;
impl RlmRepl {
    /// Execute a tree-sitter AST query on the context.
    pub(super) fn execute_ast_query(&self, query: &str) -> String {
        let mut oracle = crate::oracle::TreeSitterOracle::new(self.context.clone());

        match oracle.query(query) {
            Ok(result) => {
                if result.matches.is_empty() {
                    "(no AST matches)".to_string()
                } else {
                    let lines: Vec<String> = result
                        .matches
                        .iter()
                        .map(|m| {
                            let captures_str: Vec<String> = m
                                .captures
                                .iter()
                                .map(|(k, v)| format!("{}={:?}", k, v))
                                .collect();
                            format!("L{}: {} [{}]", m.line, m.text, captures_str.join(", "))
                        })
                        .collect();
                    lines.join("\n")
                }
            }
            Err(e) => format!("AST query error: {}", e),
        }
    }
}
