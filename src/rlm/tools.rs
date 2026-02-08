//! RLM REPL operations expressed as tool definitions.
//!
//! When FunctionGemma is active the RLM loop sends these definitions alongside
//! the analysis prompt.  The primary LLM (or FunctionGemma after reformatting)
//! returns structured `ContentPart::ToolCall` entries that are dispatched here
//! instead of being regex-parsed from code blocks.
//!
//! Each tool mirrors a command in the existing DSL REPL (`head`, `tail`,
//! `grep`, `count`, `llm_query`, `FINAL`).

use crate::provider::ToolDefinition;

/// All RLM REPL operations as tool definitions.
pub fn rlm_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "rlm_head".to_string(),
            description: "Return the first N lines of the loaded context.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "n": {
                        "type": "integer",
                        "description": "Number of lines from the start (default: 10)"
                    }
                },
                "required": []
            }),
        },
        ToolDefinition {
            name: "rlm_tail".to_string(),
            description: "Return the last N lines of the loaded context.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "n": {
                        "type": "integer",
                        "description": "Number of lines from the end (default: 10)"
                    }
                },
                "required": []
            }),
        },
        ToolDefinition {
            name: "rlm_grep".to_string(),
            description: "Search the loaded context for lines matching a regex pattern. Returns matching lines with line numbers.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "Regex pattern to search for"
                    }
                },
                "required": ["pattern"]
            }),
        },
        ToolDefinition {
            name: "rlm_count".to_string(),
            description: "Count occurrences of a regex pattern in the loaded context.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "Regex pattern to count"
                    }
                },
                "required": ["pattern"]
            }),
        },
        ToolDefinition {
            name: "rlm_slice".to_string(),
            description: "Return a slice of the context by line range.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "start": {
                        "type": "integer",
                        "description": "Start line number (0-indexed)"
                    },
                    "end": {
                        "type": "integer",
                        "description": "End line number (exclusive)"
                    }
                },
                "required": ["start", "end"]
            }),
        },
        ToolDefinition {
            name: "rlm_llm_query".to_string(),
            description: "Ask a focused sub-question about a portion of the context. Use this for semantic understanding of specific sections.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The question to answer about the context"
                    },
                    "context_slice": {
                        "type": "string",
                        "description": "Optional: specific text slice to analyze (if omitted, uses full context)"
                    }
                },
                "required": ["query"]
            }),
        },
        ToolDefinition {
            name: "rlm_final".to_string(),
            description: "Return the final answer to the analysis query. Call this when you have gathered enough information to answer.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "answer": {
                        "type": "string",
                        "description": "The complete, detailed answer to the original query"
                    }
                },
                "required": ["answer"]
            }),
        },
    ]
}

/// Result of dispatching an RLM tool call.
pub enum RlmToolResult {
    /// Normal output to feed back to the LLM.
    Output(String),
    /// The final answer — terminates the RLM loop.
    Final(String),
}

/// Dispatch a structured tool call against the REPL.
///
/// Returns `None` if the tool name is not an `rlm_*` tool (pass-through for
/// any other tool calls the model may have produced).
pub fn dispatch_tool_call(
    name: &str,
    arguments: &str,
    repl: &mut super::repl::RlmRepl,
) -> Option<RlmToolResult> {
    let args: serde_json::Value = serde_json::from_str(arguments).unwrap_or_default();

    match name {
        "rlm_head" => {
            let n = args.get("n").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
            let output = repl.head(n).join("\n");
            Some(RlmToolResult::Output(output))
        }
        "rlm_tail" => {
            let n = args.get("n").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
            let output = repl.tail(n).join("\n");
            Some(RlmToolResult::Output(output))
        }
        "rlm_grep" => {
            let pattern = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
            let matches = repl.grep(pattern);
            let output = matches
                .iter()
                .map(|(i, line)| format!("{}:{}", i, line))
                .collect::<Vec<_>>()
                .join("\n");
            if output.is_empty() {
                Some(RlmToolResult::Output("(no matches)".to_string()))
            } else {
                Some(RlmToolResult::Output(output))
            }
        }
        "rlm_count" => {
            let pattern = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
            let count = repl.count(pattern);
            Some(RlmToolResult::Output(count.to_string()))
        }
        "rlm_slice" => {
            let start = args.get("start").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            let end = args.get("end").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            let output = repl.slice(start, end).to_string();
            Some(RlmToolResult::Output(output))
        }
        "rlm_llm_query" => {
            // The llm_query tool requires async provider calls — return a
            // sentinel so the caller knows to handle it specially.
            let query = args
                .get("query")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let context_slice = args
                .get("context_slice")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            // Encode the query + optional slice as JSON so the caller can
            // destructure it.
            let payload = serde_json::json!({
                "__rlm_llm_query": true,
                "query": query,
                "context_slice": context_slice,
            });
            Some(RlmToolResult::Output(payload.to_string()))
        }
        "rlm_final" => {
            let answer = args
                .get("answer")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            Some(RlmToolResult::Final(answer))
        }
        _ => None, // Not an RLM tool
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlm::repl::{ReplRuntime, RlmRepl};

    #[test]
    fn tool_definitions_are_complete() {
        let defs = rlm_tool_definitions();
        assert_eq!(defs.len(), 7);
        let names: Vec<&str> = defs.iter().map(|d| d.name.as_str()).collect();
        assert!(names.contains(&"rlm_head"));
        assert!(names.contains(&"rlm_tail"));
        assert!(names.contains(&"rlm_grep"));
        assert!(names.contains(&"rlm_count"));
        assert!(names.contains(&"rlm_slice"));
        assert!(names.contains(&"rlm_llm_query"));
        assert!(names.contains(&"rlm_final"));
    }

    #[test]
    fn dispatch_head() {
        let ctx = "line 1\nline 2\nline 3\nline 4\nline 5".to_string();
        let mut repl = RlmRepl::new(ctx, ReplRuntime::Rust);
        let result = dispatch_tool_call("rlm_head", r#"{"n": 2}"#, &mut repl);
        match result {
            Some(RlmToolResult::Output(s)) => assert_eq!(s, "line 1\nline 2"),
            _ => panic!("expected Output"),
        }
    }

    #[test]
    fn dispatch_tail() {
        let ctx = "line 1\nline 2\nline 3\nline 4\nline 5".to_string();
        let mut repl = RlmRepl::new(ctx, ReplRuntime::Rust);
        let result = dispatch_tool_call("rlm_tail", r#"{"n": 2}"#, &mut repl);
        match result {
            Some(RlmToolResult::Output(s)) => assert_eq!(s, "line 4\nline 5"),
            _ => panic!("expected Output"),
        }
    }

    #[test]
    fn dispatch_grep() {
        let ctx = "error: fail\ninfo: ok\nerror: boom".to_string();
        let mut repl = RlmRepl::new(ctx, ReplRuntime::Rust);
        let result = dispatch_tool_call("rlm_grep", r#"{"pattern": "error"}"#, &mut repl);
        match result {
            Some(RlmToolResult::Output(s)) => {
                assert!(s.contains("error: fail"));
                assert!(s.contains("error: boom"));
            }
            _ => panic!("expected Output"),
        }
    }

    #[test]
    fn dispatch_final() {
        let ctx = "whatever".to_string();
        let mut repl = RlmRepl::new(ctx, ReplRuntime::Rust);
        let result = dispatch_tool_call(
            "rlm_final",
            r#"{"answer": "The answer is 42"}"#,
            &mut repl,
        );
        match result {
            Some(RlmToolResult::Final(s)) => assert_eq!(s, "The answer is 42"),
            _ => panic!("expected Final"),
        }
    }

    #[test]
    fn dispatch_unknown_returns_none() {
        let ctx = "data".to_string();
        let mut repl = RlmRepl::new(ctx, ReplRuntime::Rust);
        assert!(dispatch_tool_call("unknown_tool", "{}", &mut repl).is_none());
    }
}
