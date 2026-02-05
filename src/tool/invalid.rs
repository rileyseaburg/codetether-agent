//! Invalid tool: Handle invalid or unknown tool calls gracefully

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

/// Invalid Tool - A fallback tool that is returned when a model tries to call
/// a tool that doesn't exist. This provides helpful error messages and suggestions.
pub struct InvalidTool {
    requested_tool: String,
    available_tools: Vec<String>,
}

impl Default for InvalidTool {
    fn default() -> Self {
        Self::new()
    }
}

impl InvalidTool {
    pub fn new() -> Self {
        Self {
            requested_tool: String::new(),
            available_tools: Vec::new(),
        }
    }

    pub fn with_context(requested_tool: String, available_tools: Vec<String>) -> Self {
        Self {
            requested_tool,
            available_tools,
        }
    }
}

#[async_trait]
impl Tool for InvalidTool {
    fn id(&self) -> &str {
        "invalid"
    }

    fn name(&self) -> &str {
        "Invalid Tool Handler"
    }

    fn description(&self) -> &str {
        "Handles invalid or unknown tool calls. Returns helpful error messages with suggestions for valid tools."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "requested_tool": {
                    "type": "string",
                    "description": "The tool that was requested but not found"
                },
                "args": {
                    "type": "object",
                    "description": "The arguments that were passed"
                }
            },
            "required": ["requested_tool"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let requested = args["requested_tool"]
            .as_str()
            .unwrap_or(&self.requested_tool);

        let passed_args = if let Some(obj) = args.get("args") {
            serde_json::to_string_pretty(obj).unwrap_or_else(|_| "{}".to_string())
        } else {
            "{}".to_string()
        };

        let suggestions = if !self.available_tools.is_empty() {
            // Find similar tool names
            let similar: Vec<&String> = self
                .available_tools
                .iter()
                .filter(|t| {
                    t.contains(requested)
                        || requested.contains(t.as_str())
                        || Self::levenshtein(t, requested) <= 3
                })
                .take(3)
                .collect();

            if similar.is_empty() {
                format!("Available tools: {}", self.available_tools.join(", "))
            } else {
                format!(
                    "Did you mean: {}?\n\nAll available tools: {}",
                    similar
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    self.available_tools.join(", ")
                )
            }
        } else {
            "No tool list provided. Check available tools.".to_string()
        };

        let message = format!(
            "Tool '{}' not found.\n\n\
            Arguments received: {}\n\n\
            {}",
            requested, passed_args, suggestions
        );

        Ok(ToolResult::error(message))
    }
}

impl InvalidTool {
    /// Simple Levenshtein distance for finding similar tool names
    fn levenshtein(a: &str, b: &str) -> usize {
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        let m = a.len();
        let n = b.len();

        if m == 0 {
            return n;
        }
        if n == 0 {
            return m;
        }

        let mut prev: Vec<usize> = (0..=n).collect();
        let mut curr = vec![0; n + 1];

        for i in 1..=m {
            curr[0] = i;
            for j in 1..=n {
                let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
                curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        prev[n]
    }
}
