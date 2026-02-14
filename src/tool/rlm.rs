//! RLM tool: Recursive Language Model for large context analysis
//!
//! This tool invokes the RLM subsystem to process large codebases that exceed
//! the context window. It chunks, routes, and synthesizes results.

use super::{Tool, ToolResult};
use crate::provider::Provider;
use crate::rlm::router::AutoProcessContext;
use crate::rlm::{RlmChunker, RlmConfig, RlmRouter};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;

/// RLM Tool - Invoke the Recursive Language Model subsystem
/// for analyzing large codebases that exceed the context window
pub struct RlmTool {
    provider: Arc<dyn Provider>,
    model: String,
}

impl RlmTool {
    pub fn new(provider: Arc<dyn Provider>, model: String) -> Self {
        Self { provider, model }
    }
}

#[async_trait]
impl Tool for RlmTool {
    fn id(&self) -> &str {
        "rlm"
    }

    fn name(&self) -> &str {
        "RLM"
    }

    fn description(&self) -> &str {
        "Recursive Language Model for processing large codebases. Use this when you need to analyze files or content that exceeds the context window. RLM chunks the content, processes each chunk, and synthesizes results. Actions: 'analyze' (analyze large content), 'summarize' (summarize large files), 'search' (semantic search across large codebase)."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "Action: 'analyze' (deep analysis), 'summarize' (generate summary), 'search' (semantic search)",
                    "enum": ["analyze", "summarize", "search"]
                },
                "query": {
                    "type": "string",
                    "description": "The question or query to answer (for analyze/search)"
                },
                "paths": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "File or directory paths to process"
                },
                "content": {
                    "type": "string",
                    "description": "Direct content to analyze (alternative to paths)"
                },
                "max_depth": {
                    "type": "integer",
                    "description": "Maximum recursion depth (default: 3)",
                    "default": 3
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let action = args["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("action is required"))?;

        let query = args["query"].as_str().unwrap_or("");
        let paths: Vec<&str> = args["paths"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        let content = args["content"].as_str();

        match action {
            "analyze" | "summarize" | "search" => {
                if action != "summarize" && query.is_empty() {
                    return Ok(ToolResult::error(format!(
                        "query is required for '{}' action",
                        action
                    )));
                }

                // Collect content from paths or direct content
                let all_content = if let Some(c) = content {
                    c.to_string()
                } else if !paths.is_empty() {
                    let mut collected = String::new();
                    for path in &paths {
                        match tokio::fs::read_to_string(path).await {
                            Ok(c) => {
                                collected.push_str(&format!("=== {} ===\n{}\n\n", path, c));
                            }
                            Err(e) => {
                                collected.push_str(&format!("=== {} (error: {}) ===\n\n", path, e));
                            }
                        }
                    }
                    collected
                } else {
                    return Ok(ToolResult::error("Either 'paths' or 'content' is required"));
                };

                let input_tokens = RlmChunker::estimate_tokens(&all_content);
                let effective_query = if query.is_empty() {
                    format!("Summarize the content from: {:?}", paths)
                } else {
                    query.to_string()
                };

                // Use RlmRouter::auto_process for real analysis
                let auto_ctx = AutoProcessContext {
                    tool_id: action,
                    tool_args: json!({ "query": effective_query, "paths": paths }),
                    session_id: "rlm-tool",
                    abort: None,
                    on_progress: None,
                    provider: Arc::clone(&self.provider),
                    model: self.model.clone(),
                };
                let config = RlmConfig::default();

                match RlmRouter::auto_process(&all_content, auto_ctx, &config).await {
                    Ok(result) => {
                        let output = format!(
                            "RLM {} complete ({} → {} tokens, {} iterations)\n\n{}",
                            action,
                            result.stats.input_tokens,
                            result.stats.output_tokens,
                            result.stats.iterations,
                            result.processed
                        );
                        Ok(ToolResult::success(output))
                    }
                    Err(e) => {
                        // Fallback to smart truncation
                        tracing::warn!(error = %e, "RLM auto_process failed, falling back to truncation");
                        let (truncated, _, _) = RlmRouter::smart_truncate(
                            &all_content,
                            action,
                            &json!({}),
                            input_tokens.min(8000),
                        );
                        Ok(ToolResult::success(format!(
                            "RLM {} (fallback mode - auto_process failed: {})\n\n{}",
                            action, e, truncated
                        )))
                    }
                }
            }
            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Use 'analyze', 'summarize', or 'search'.",
                action
            ))),
        }
    }
}
