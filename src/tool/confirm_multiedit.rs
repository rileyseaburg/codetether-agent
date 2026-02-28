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

impl Default for ConfirmMultiEditTool {
    fn default() -> Self {
        Self::new()
    }
}

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
        let edits_val = match input.get("edits").and_then(|v| v.as_array()) {
            Some(arr) if !arr.is_empty() => arr,
            Some(_) => {
                return Ok(ToolResult::structured_error(
                    "INVALID_FIELD",
                    "confirm_multiedit",
                    "edits array must contain at least one edit operation",
                    Some(vec!["edits"]),
                    Some(
                        json!({"edits": [{"file": "path/to/file", "old_string": "old", "new_string": "new"}]}),
                    ),
                ));
            }
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "confirm_multiedit",
                    "edits is required and must be an array of edit objects with 'file', 'old_string', 'new_string' fields",
                    Some(vec!["edits"]),
                    Some(
                        json!({"edits": [{"file": "path/to/file", "old_string": "old", "new_string": "new"}]}),
                    ),
                ));
            }
        };

        let mut edits = Vec::new();
        for (i, edit_val) in edits_val.iter().enumerate() {
            let file = match edit_val.get("file").and_then(|v| v.as_str()) {
                Some(s) => s.to_string(),
                None => {
                    return Ok(ToolResult::structured_error(
                        "INVALID_FIELD",
                        "confirm_multiedit",
                        &format!("edits[{i}].file is required"),
                        Some(vec!["file"]),
                        Some(
                            json!({"file": "path/to/file", "old_string": "old", "new_string": "new"}),
                        ),
                    ));
                }
            };
            let old_string = match edit_val.get("old_string").and_then(|v| v.as_str()) {
                Some(s) => s.to_string(),
                None => {
                    return Ok(ToolResult::structured_error(
                        "INVALID_FIELD",
                        "confirm_multiedit",
                        &format!("edits[{i}].old_string is required"),
                        Some(vec!["old_string"]),
                        Some(
                            json!({"file": file, "old_string": "text to find", "new_string": "replacement"}),
                        ),
                    ));
                }
            };
            let new_string = match edit_val.get("new_string").and_then(|v| v.as_str()) {
                Some(s) => s.to_string(),
                None => {
                    return Ok(ToolResult::structured_error(
                        "INVALID_FIELD",
                        "confirm_multiedit",
                        &format!("edits[{i}].new_string is required"),
                        Some(vec!["new_string"]),
                        Some(
                            json!({"file": file, "old_string": old_string, "new_string": "replacement"}),
                        ),
                    ));
                }
            };
            edits.push(EditOperation {
                file,
                old_string,
                new_string,
            });
        }

        let confirm = input.get("confirm").and_then(|v| v.as_bool());

        if edits.is_empty() {
            return Ok(ToolResult::error("No edits provided"));
        }

        // Phase 1: Validation - read all files and check that old_string exists uniquely
        let mut file_contents: Vec<(PathBuf, String, String, String)> = Vec::new();
        let mut previews: Vec<EditPreview> = Vec::new();

        for edit in &edits {
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
                    truncate_with_ellipsis(&edit.old_string, 50)
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
                    diff_output.push_str(line.trim_end());
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
        if confirm.is_none() {
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
        if confirm == Some(true) {
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
