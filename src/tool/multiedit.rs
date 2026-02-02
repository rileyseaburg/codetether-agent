//! Multi-Edit Tool
//!
//! Edit multiple files atomically with an array of replacements.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use tokio::fs;

use super::{Tool, ToolResult};

pub struct MultiEditTool;

impl Default for MultiEditTool {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiEditTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
struct MultiEditParams {
    edits: Vec<EditOperation>,
}

#[derive(Debug, Deserialize)]
struct EditOperation {
    file: String,
    old_string: String,
    new_string: String,
}

#[derive(Debug, serde::Serialize)]
struct EditResult {
    file: String,
    success: bool,
    message: String,
}

#[async_trait]
impl Tool for MultiEditTool {
    fn id(&self) -> &str {
        "multiedit"
    }

    fn name(&self) -> &str {
        "Multi Edit"
    }

    fn description(&self) -> &str {
        "Edit multiple files atomically. Each edit replaces an old string with a new string. \
         All edits are validated before any changes are applied. If any edit fails validation, \
         no changes are made."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "edits": {
                    "type": "array",
                    "description": "Array of edit operations to apply",
                    "items": {
                        "type": "object",
                        "properties": {
                            "file": {
                                "type": "string",
                                "description": "Path to the file to edit"
                            },
                            "old_string": {
                                "type": "string",
                                "description": "The exact string to find and replace"
                            },
                            "new_string": {
                                "type": "string",
                                "description": "The string to replace it with"
                            }
                        },
                        "required": ["file", "old_string", "new_string"]
                    }
                }
            },
            "required": ["edits"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: MultiEditParams = serde_json::from_value(params)
            .context("Invalid parameters")?;

        if params.edits.is_empty() {
            return Ok(ToolResult::error("No edits provided"));
        }

        // Phase 1: Validation - read all files and check that old_string exists uniquely
        let mut file_contents: Vec<(PathBuf, String, String, String)> = Vec::new();
        
        for edit in &params.edits {
            let path = PathBuf::from(&edit.file);
            
            if !path.exists() {
                return Ok(ToolResult::error(format!(
                    "File does not exist: {}",
                    edit.file
                )));
            }

            let content = fs::read_to_string(&path)
                .await
                .with_context(|| format!("Failed to read file: {}", edit.file))?;

            // Check that old_string exists exactly once
            let matches: Vec<_> = content.match_indices(&edit.old_string).collect();
            
            if matches.is_empty() {
                return Ok(ToolResult::error(format!(
                    "String not found in {}: {}",
                    edit.file,
                    if edit.old_string.len() > 50 {
                        format!("{}...", &edit.old_string[..50])
                    } else {
                        edit.old_string.clone()
                    }
                )));
            }
            
            if matches.len() > 1 {
                return Ok(ToolResult::error(format!(
                    "String found {} times in {} (must be unique). Use more context to disambiguate.",
                    matches.len(),
                    edit.file
                )));
            }

            file_contents.push((
                path,
                content,
                edit.old_string.clone(),
                edit.new_string.clone(),
            ));
        }

        // Phase 2: Apply all edits
        let mut results: Vec<EditResult> = Vec::new();
        
        for (path, content, old_string, new_string) in file_contents {
            let new_content = content.replacen(&old_string, &new_string, 1);
            
            fs::write(&path, &new_content)
                .await
                .with_context(|| format!("Failed to write file: {}", path.display()))?;

            results.push(EditResult {
                file: path.display().to_string(),
                success: true,
                message: format!(
                    "Replaced {} chars with {} chars",
                    old_string.len(),
                    new_string.len()
                ),
            });
        }

        let output = format!(
            "Successfully applied {} edits:\n{}",
            results.len(),
            results
                .iter()
                .map(|r| format!("  ✓ {}: {}", r.file, r.message))
                .collect::<Vec<_>>()
                .join("\n")
        );

        Ok(ToolResult::success(output)
            .with_metadata("edits", json!(results)))
    }
}
