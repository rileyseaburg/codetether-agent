//! Execute rlm_ast_query against normalized JSON arguments.
use super::super::RlmToolResult;
use super::super::output::clamp_tool_output;
use crate::repl::RlmRepl;
use serde_json::Value;
pub(super) fn execute(args: &Value, repl: &mut RlmRepl) -> Option<RlmToolResult> {
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");

    // Create a tree-sitter oracle and execute the query
    let mut oracle = crate::oracle::TreeSitterOracle::new(repl.context().to_string());
    match oracle.query(query) {
        Ok(result) => {
            // Format the result as JSON
            let matches: Vec<serde_json::Value> = result
                .matches
                .iter()
                .map(|m| {
                    serde_json::json!({
                        "line": m.line,
                        "column": m.column,
                        "captures": m.captures,
                        "text": m.text
                    })
                })
                .collect();

            let output = serde_json::json!({
                "query": query,
                "match_count": matches.len(),
                "matches": matches
            });

            Some(RlmToolResult::Output(clamp_tool_output(
                output.to_string(),
                "rlm_ast_query",
            )))
        }
        Err(e) => Some(RlmToolResult::Output(format!("AST query error: {}", e))),
    }
}
