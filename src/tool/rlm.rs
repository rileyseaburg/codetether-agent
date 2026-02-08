//! RLM tool: Recursive Language Model for large context analysis
//!
//! This tool invokes the RLM subsystem to process large codebases that exceed
//! the context window. It chunks, routes, and synthesizes results.

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

/// RLM Tool - Invoke the Recursive Language Model subsystem
/// for analyzing large codebases that exceed the context window
pub struct RlmTool {
    max_chunk_size: usize,
}

impl Default for RlmTool {
    fn default() -> Self {
        Self::new()
    }
}

impl RlmTool {
    pub fn new() -> Self {
        Self {
            max_chunk_size: 8192,
        }
    }

    #[allow(dead_code)]
    pub fn with_chunk_size(max_chunk_size: usize) -> Self {
        Self { max_chunk_size }
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
        let max_depth = args["max_depth"].as_u64().unwrap_or(3) as usize;

        match action {
            "analyze" => {
                if query.is_empty() {
                    return Ok(ToolResult::error("query is required for 'analyze' action"));
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

                // For now, return a chunked analysis placeholder
                // Full implementation would invoke the RLM subsystem
                let chunks = self.chunk_content(&all_content);
                let first_chunk_preview = chunks
                    .first()
                    .map(|chunk| truncate_with_ellipsis(chunk, 500))
                    .unwrap_or_default();
                let output = format!(
                    "RLM Analysis\n\
                    Query: {}\n\
                    Paths: {:?}\n\
                    Content size: {} bytes\n\
                    Chunks: {}\n\
                    Max depth: {}\n\n\
                    [Full RLM processing would analyze each chunk and synthesize results]\n\n\
                    Content preview (first chunk):\n{}",
                    query,
                    paths,
                    all_content.len(),
                    chunks.len(),
                    max_depth,
                    first_chunk_preview
                );

                Ok(ToolResult::success(output))
            }
            "summarize" => {
                if paths.is_empty() && content.is_none() {
                    return Ok(ToolResult::error("Either 'paths' or 'content' is required"));
                }

                let all_content = if let Some(c) = content {
                    c.to_string()
                } else {
                    let mut collected = String::new();
                    for path in &paths {
                        match tokio::fs::read_to_string(path).await {
                            Ok(c) => collected.push_str(&c),
                            Err(e) => {
                                collected.push_str(&format!("[Error reading {}: {}]\n", path, e))
                            }
                        }
                    }
                    collected
                };

                let chunks = self.chunk_content(&all_content);
                let output = format!(
                    "RLM Summary\n\
                    Paths: {:?}\n\
                    Content size: {} bytes\n\
                    Chunks: {}\n\n\
                    [Full RLM would summarize each chunk and combine summaries]",
                    paths,
                    all_content.len(),
                    chunks.len()
                );

                Ok(ToolResult::success(output))
            }
            "search" => {
                if query.is_empty() {
                    return Ok(ToolResult::error("query is required for 'search' action"));
                }

                let output = format!(
                    "RLM Semantic Search\n\
                    Query: {}\n\
                    Paths: {:?}\n\n\
                    [Full RLM would perform semantic search across chunks]",
                    query, paths
                );

                Ok(ToolResult::success(output))
            }
            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Use 'analyze', 'summarize', or 'search'.",
                action
            ))),
        }
    }
}

impl RlmTool {
    fn chunk_content(&self, content: &str) -> Vec<String> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_chunk = String::new();

        for line in lines {
            if current_chunk.len() + line.len() + 1 > self.max_chunk_size
                && !current_chunk.is_empty()
            {
                chunks.push(current_chunk);
                current_chunk = String::new();
            }
            current_chunk.push_str(line);
            current_chunk.push('\n');
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        chunks
    }
}

fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = value.chars();
    let mut output = String::new();
    for _ in 0..max_chars {
        if let Some(ch) = chars.next() {
            output.push(ch);
        } else {
            return value.to_string();
        }
    }

    if chars.next().is_some() {
        format!("{output}...")
    } else {
        output
    }
}
