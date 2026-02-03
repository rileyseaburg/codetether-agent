//! Search tools: grep

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use serde_json::{json, Value};
use ignore::WalkBuilder;

/// Search for text in files
pub struct GrepTool;

impl GrepTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for GrepTool {
    fn id(&self) -> &str {
        "grep"
    }

    fn name(&self) -> &str {
        "Grep Search"
    }

    fn description(&self) -> &str {
        "Search for text or regex patterns in files. Respects .gitignore by default."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The text or regex pattern to search for"
                },
                "path": {
                    "type": "string",
                    "description": "Directory or file to search in (default: current directory)"
                },
                "is_regex": {
                    "type": "boolean",
                    "description": "Whether the pattern is a regex (default: false)"
                },
                "include": {
                    "type": "string",
                    "description": "Glob pattern to include files (e.g., *.rs)"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of matches to return"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let pattern = args["pattern"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("pattern is required"))?;
        let search_path = args["path"].as_str().unwrap_or(".");
        let is_regex = args["is_regex"].as_bool().unwrap_or(false);
        let include = args["include"].as_str();
        let limit = args["limit"].as_u64().unwrap_or(50) as usize;

        let regex = if is_regex {
            Regex::new(pattern)?
        } else {
            Regex::new(&regex::escape(pattern))?
        };

        let mut results = Vec::new();
        let mut walker = WalkBuilder::new(search_path);
        walker.hidden(false).git_ignore(true);

        for entry in walker.build() {
            if results.len() >= limit {
                break;
            }

            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                continue;
            }

            let path = entry.path();

            // Check include pattern
            if let Some(include_pattern) = include {
                if !glob::Pattern::new(include_pattern)
                    .map(|p| p.matches_path(path))
                    .unwrap_or(false)
                {
                    continue;
                }
            }

            // Read and search file
            if let Ok(content) = tokio::fs::read_to_string(path).await {
                for (line_num, line) in content.lines().enumerate() {
                    if results.len() >= limit {
                        break;
                    }

                    if regex.is_match(line) {
                        results.push(format!(
                            "{}:{}: {}",
                            path.display(),
                            line_num + 1,
                            line.trim()
                        ));
                    }
                }
            }
        }

        let truncated = results.len() >= limit;
        let output = results.join("\n");

        Ok(ToolResult::success(output)
            .with_metadata("count", json!(results.len()))
            .with_metadata("truncated", json!(truncated)))
    }
}
