//! File tools: read, write, list, glob

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::path::PathBuf;
use tokio::fs;

/// Read file contents
pub struct ReadTool;

impl ReadTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for ReadTool {
    fn id(&self) -> &str {
        "read"
    }

    fn name(&self) -> &str {
        "Read File"
    }

    fn description(&self) -> &str {
        "Read the contents of a file. Provide the file path to read."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to read"
                },
                "offset": {
                    "type": "integer",
                    "description": "Line number to start reading from (1-indexed)"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of lines to read"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("path is required"))?;
        let offset = args["offset"].as_u64().map(|n| n as usize);
        let limit = args["limit"].as_u64().map(|n| n as usize);

        let content = fs::read_to_string(path).await?;
        
        let lines: Vec<&str> = content.lines().collect();
        let start = offset.map(|o| o.saturating_sub(1)).unwrap_or(0);
        let end = limit.map(|l| (start + l).min(lines.len())).unwrap_or(lines.len());
        
        let selected: String = lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{:4} | {}", start + i + 1, line))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(ToolResult::success(selected)
            .with_metadata("total_lines", json!(lines.len()))
            .with_metadata("read_lines", json!(end - start)))
    }
}

/// Write file contents
pub struct WriteTool;

impl WriteTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for WriteTool {
    fn id(&self) -> &str {
        "write"
    }

    fn name(&self) -> &str {
        "Write File"
    }

    fn description(&self) -> &str {
        "Write content to a file. Creates the file if it doesn't exist, or overwrites it."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("path is required"))?;
        let content = args["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("content is required"))?;

        // Create parent directories if needed
        if let Some(parent) = PathBuf::from(path).parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(path, content).await?;

        Ok(ToolResult::success(format!("Wrote {} bytes to {}", content.len(), path)))
    }
}

/// List directory contents
pub struct ListTool;

impl ListTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for ListTool {
    fn id(&self) -> &str {
        "list"
    }

    fn name(&self) -> &str {
        "List Directory"
    }

    fn description(&self) -> &str {
        "List the contents of a directory."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the directory to list"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("path is required"))?;

        let mut entries = fs::read_dir(path).await?;
        let mut items = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let name = entry.file_name().to_string_lossy().to_string();
            let file_type = entry.file_type().await?;
            
            let suffix = if file_type.is_dir() {
                "/"
            } else if file_type.is_symlink() {
                "@"
            } else {
                ""
            };
            
            items.push(format!("{}{}", name, suffix));
        }

        items.sort();
        Ok(ToolResult::success(items.join("\n"))
            .with_metadata("count", json!(items.len())))
    }
}

/// Find files using glob patterns
pub struct GlobTool;

impl GlobTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for GlobTool {
    fn id(&self) -> &str {
        "glob"
    }

    fn name(&self) -> &str {
        "Glob Search"
    }

    fn description(&self) -> &str {
        "Find files matching a glob pattern (e.g., **/*.rs, src/**/*.ts)"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The glob pattern to match files"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of results to return"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let pattern = args["pattern"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("pattern is required"))?;
        let limit = args["limit"].as_u64().unwrap_or(100) as usize;

        let mut matches = Vec::new();
        
        for entry in glob::glob(pattern)? {
            if matches.len() >= limit {
                break;
            }
            if let Ok(path) = entry {
                matches.push(path.display().to_string());
            }
        }

        let truncated = matches.len() >= limit;
        let output = matches.join("\n");

        Ok(ToolResult::success(output)
            .with_metadata("count", json!(matches.len()))
            .with_metadata("truncated", json!(truncated)))
    }
}
