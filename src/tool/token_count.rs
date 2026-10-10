//! `token_count` tool: estimate a file's token cost before reading it.
//!
//! Lets the agent decide whether to `read` a whole file, read a slice
//! with `offset`/`limit`, or route it through `rlm` instead.

use super::{Tool, ToolResult};
use crate::rlm::RlmChunker;
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{Value, json};

/// Reports estimated token count, bytes, and lines for one or more files.
pub struct TokenCountTool;

#[async_trait]
impl Tool for TokenCountTool {
    fn id(&self) -> &str {
        "token_count"
    }

    fn name(&self) -> &str {
        "Token Count"
    }

    fn description(&self) -> &str {
        "token_count(paths: string[]) - Estimate token cost of files BEFORE reading them. \
         Use on large or unknown files; if large, read a range or use rlm."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "paths": {"type": "array", "items": {"type": "string"},
                          "description": "Files to measure"}
            },
            "required": ["paths"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let paths: Vec<String> =
            serde_json::from_value(args["paths"].clone()).context("paths must be string[]")?;
        let mut lines = Vec::with_capacity(paths.len());
        for path in paths {
            lines.push(match tokio::fs::read(&path).await {
                Ok(bytes) => describe(&path, &String::from_utf8_lossy(&bytes)),
                Err(e) => format!("{path}: error {e}"),
            });
        }
        Ok(ToolResult::success(lines.join("\n")))
    }
}

/// Format one file's measurements; public for testing.
pub fn describe(path: &str, text: &str) -> String {
    let tokens = RlmChunker::estimate_tokens(text);
    format!("{path}: ~{tokens} tokens, {} bytes, {} lines", text.len(), text.lines().count())
}
