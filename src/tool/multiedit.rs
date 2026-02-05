//! Multi-Edit Tool
//!
//! Edit multiple files atomically with an array of replacements.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;
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

/// Result of a single file edit operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EditResult {
    pub file: String,
    pub success: bool,
    pub message: String,
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
        let params: MultiEditParams =
            serde_json::from_value(params).context("Invalid parameters")?;

        if params.edits.is_empty() {
            return Ok(ToolResult::error("No edits provided"));
        }

        // Track results for each file edit
        let mut edit_results: Vec<EditResult> = Vec::new();
        let mut file_contents: Vec<(PathBuf, String, String, String)> = Vec::new();
        let mut previews: Vec<Value> = Vec::new();
        let mut total_added = 0;
        let mut total_removed = 0;

        // Phase 1: Validate all edits and collect file contents
        for edit in &params.edits {
            let path = PathBuf::from(&edit.file);

            // Check if file exists
            if !path.exists() {
                edit_results.push(EditResult {
                    file: edit.file.clone(),
                    success: false,
                    message: format!("File does not exist: {}", edit.file),
                });
                continue;
            }

            // Read file content
            let content = match fs::read_to_string(&path).await {
                Ok(c) => c,
                Err(e) => {
                    edit_results.push(EditResult {
                        file: edit.file.clone(),
                        success: false,
                        message: format!("Failed to read file: {}", e),
                    });
                    continue;
                }
            };

            // Check that old_string exists exactly once
            let matches: Vec<_> = content.match_indices(&edit.old_string).collect();

            if matches.is_empty() {
                let preview = if edit.old_string.len() > 50 {
                    format!("{}...", &edit.old_string[..50])
                } else {
                    edit.old_string.clone()
                };
                edit_results.push(EditResult {
                    file: edit.file.clone(),
                    success: false,
                    message: format!("String not found: {}", preview),
                });
                continue;
            }

            if matches.len() > 1 {
                edit_results.push(EditResult {
                    file: edit.file.clone(),
                    success: false,
                    message: format!(
                        "String found {} times (must be unique). Use more context to disambiguate.",
                        matches.len()
                    ),
                });
                continue;
            }

            // Validation passed - store for processing
            file_contents.push((
                path.clone(),
                content.clone(),
                edit.old_string.clone(),
                edit.new_string.clone(),
            ));

            // Generate diff preview
            let new_content = content.replacen(&edit.old_string, &edit.new_string, 1);
            let diff = TextDiff::from_lines(&content, &new_content);

            let mut diff_output = String::new();
            let mut added = 0;
            let mut removed = 0;

            for change in diff.iter_all_changes() {
                let (sign, style) = match change.tag() {
                    ChangeTag::Delete => {
                        removed += 1;
                        ("-", "red")
                    }
                    ChangeTag::Insert => {
                        added += 1;
                        ("+", "green")
                    }
                    ChangeTag::Equal => (" ", "default"),
                };

                let line = format!("{}{}", sign, change);
                if style == "red" {
                    diff_output.push_str(&format!("\x1b[31m{}\x1b[0m", line.trim_end()));
                } else if style == "green" {
                    diff_output.push_str(&format!("\x1b[32m{}\x1b[0m", line.trim_end()));
                } else {
                    diff_output.push_str(&line.trim_end());
                }
                diff_output.push('\n');
            }

            previews.push(json!({
                "file": path.display().to_string(),
                "diff": diff_output.trim(),
                "added": added,
                "removed": removed
            }));

            total_added += added;
            total_removed += removed;

            // Mark as success for validation phase
            edit_results.push(EditResult {
                file: edit.file.clone(),
                success: true,
                message: format!("Validated: +{} lines, -{} lines", added, removed),
            });
        }

        // Check if any edits failed validation
        let failed_edits: Vec<&EditResult> = edit_results.iter().filter(|r| !r.success).collect();
        let successful_edits: Vec<&EditResult> =
            edit_results.iter().filter(|r| r.success).collect();

        if !failed_edits.is_empty() {
            // Build error summary
            let mut error_summary = String::new();
            for result in &failed_edits {
                error_summary.push_str(&format!("\n✗ {}: {}", result.file, result.message));
            }

            return Ok(ToolResult {
                output: format!(
                    "Validation failed for {} of {} edits:{}\n\nNo changes were applied.",
                    failed_edits.len(),
                    params.edits.len(),
                    error_summary
                ),
                success: false,
                metadata: {
                    let mut m = HashMap::new();
                    m.insert("edit_results".to_string(), json!(edit_results));
                    m.insert("failed_count".to_string(), json!(failed_edits.len()));
                    m.insert("success_count".to_string(), json!(successful_edits.len()));
                    m
                },
            });
        }

        // All validations passed - return confirmation prompt with structured results
        let mut all_diffs = String::new();
        for preview in &previews {
            let file = preview["file"].as_str().unwrap();
            let diff = preview["diff"].as_str().unwrap();
            all_diffs.push_str(&format!("\n=== {} ===\n{}", file, diff));
        }

        // Build summary of all edits
        let mut edit_summary = String::new();
        for result in &edit_results {
            edit_summary.push_str(&format!("\n✓ {}: {}", result.file, result.message));
        }

        let mut metadata = HashMap::new();
        metadata.insert("requires_confirmation".to_string(), json!(true));
        metadata.insert("total_files".to_string(), json!(file_contents.len()));
        metadata.insert("total_added".to_string(), json!(total_added));
        metadata.insert("total_removed".to_string(), json!(total_removed));
        metadata.insert("previews".to_string(), json!(previews));
        metadata.insert("edit_results".to_string(), json!(edit_results));

        Ok(ToolResult {
            output: format!(
                "Multi-file changes require confirmation:{}{}{}\n\nTotal: {} files, +{} lines, -{} lines",
                all_diffs,
                if edit_summary.is_empty() {
                    ""
                } else {
                    "\n\nEdit summary:"
                },
                edit_summary,
                file_contents.len(),
                total_added,
                total_removed
            ),
            success: true,
            metadata,
        })
    }
}
