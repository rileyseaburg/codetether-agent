//! Confirm Multi-Edit Tool
//!
//! Edit multiple files with user confirmation via diff display

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

use super::{Tool, ToolResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmMultiEditInput {
    pub edits: Vec<EditOperation>,
    pub confirm: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditOperation {
    pub file: String,
    pub old_string: String,
    pub new_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditPreview {
    pub file: String,
    pub diff: String,
    pub added: usize,
    pub removed: usize,
}

pub struct ConfirmMultiEditTool;

impl ConfirmMultiEditTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for ConfirmMultiEditTool {
    fn id(&self) -> &str {
        "confirm_multiedit"
    }

    fn name(&self) -> &str {
        "Confirm Multi Edit"
    }

    fn description(&self) -> &str {
        "Edit multiple files with confirmation. Shows diffs for all changes and requires user confirmation before applying."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "edits": {
                    "type": "array",
                    "description": "Array of edit operations to preview",
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
                },
                "confirm": {
                    "type": "boolean",
                    "description": "Set to true to confirm and apply all changes, false to reject all",
                    "default": null
                }
            },
            "required": ["edits"]
        })
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let params: ConfirmMultiEditInput = serde_json::from_value(input)?;

        if params.edits.is_empty() {
            return Ok(ToolResult::error("No edits provided"));
        }

        // Phase 1: Validation - read all files and check that old_string exists uniquely
        let mut file_contents: Vec<(PathBuf, String, String, String)> = Vec::new();
        let mut previews: Vec<EditPreview> = Vec::new();

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

        // Phase 2: Generate diffs for all changes
        let mut total_added = 0;
        let mut total_removed = 0;

        for (path, content, old_string, new_string) in &file_contents {
            let new_content = content.replacen(old_string, new_string, 1);
            let diff = TextDiff::from_lines(content, &new_content);

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

            previews.push(EditPreview {
                file: path.display().to_string(),
                diff: diff_output.trim().to_string(),
                added,
                removed,
            });

            total_added += added;
            total_removed += removed;
        }

        // If no confirmation provided, return diffs for review
        if params.confirm.is_none() {
            let mut all_diffs = String::new();
            for preview in &previews {
                all_diffs.push_str(&format!("\n=== {} ===\n{}", preview.file, preview.diff));
            }

            let mut metadata = HashMap::new();
            metadata.insert("requires_confirmation".to_string(), json!(true));
            metadata.insert("total_files".to_string(), json!(previews.len()));
            metadata.insert("total_added".to_string(), json!(total_added));
            metadata.insert("total_removed".to_string(), json!(total_removed));
            metadata.insert("previews".to_string(), json!(previews));

            return Ok(ToolResult {
                output: format!(
                    "Multi-file changes require confirmation:{}\n\nTotal: {} files, +{} lines, -{} lines",
                    all_diffs,
                    previews.len(),
                    total_added,
                    total_removed
                ),
                success: true,
                metadata,
            });
        }

        // Handle confirmation
        if params.confirm == Some(true) {
            // Apply all changes
            for (path, content, old_string, new_string) in file_contents {
                let new_content = content.replacen(&old_string, &new_string, 1);
                fs::write(&path, &new_content).await?;
            }

            Ok(ToolResult::success(format!(
                "✓ Applied {} file changes",
                previews.len()
            )))
        } else {
            Ok(ToolResult::success("✗ All changes rejected by user"))
        }
    }
}
