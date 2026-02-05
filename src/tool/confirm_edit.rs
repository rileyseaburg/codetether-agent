//! Confirm Edit Tool
//!
//! Edit files with user confirmation via diff display

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;
use std::time::Instant;
use tokio::fs;

use super::{Tool, ToolResult};
use crate::telemetry::{FileChange, ToolExecution, TOOL_EXECUTIONS, record_persistent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmEditInput {
    pub path: String,
    pub old_string: String,
    pub new_string: String,
    pub confirm: Option<bool>,
}

pub struct ConfirmEditTool;

impl ConfirmEditTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for ConfirmEditTool {
    fn id(&self) -> &str {
        "confirm_edit"
    }

    fn name(&self) -> &str {
        "Confirm Edit"
    }

    fn description(&self) -> &str {
        "Edit files with confirmation. Shows diff and requires user confirmation before applying changes."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to edit"
                },
                "old_string": {
                    "type": "string",
                    "description": "The exact string to replace"
                },
                "new_string": {
                    "type": "string",
                    "description": "The string to replace with"
                },
                "confirm": {
                    "type": "boolean",
                    "description": "Set to true to confirm and apply changes, false to reject",
                    "default": null
                }
            },
            "required": ["path", "old_string", "new_string"]
        })
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let params: ConfirmEditInput = serde_json::from_value(input)?;

        let path = &params.path;
        let old_string = &params.old_string;
        let new_string = &params.new_string;

        // Read the file
        let content = fs::read_to_string(path).await?;

        // Count occurrences
        let count = content.matches(old_string).count();

        if count == 0 {
            return Ok(ToolResult::structured_error(
                "NOT_FOUND",
                "confirm_edit",
                "old_string not found in file. Make sure it matches exactly, including whitespace.",
                None,
                Some(json!({
                    "hint": "Use the 'read' tool first to see the exact content",
                    "path": path,
                    "old_string": "<copy exact text from file>",
                    "new_string": new_string
                })),
            ));
        }

        if count > 1 {
            return Ok(ToolResult::structured_error(
                "AMBIGUOUS_MATCH",
                "confirm_edit",
                &format!(
                    "old_string found {} times. Include more context to uniquely identify the location.",
                    count
                ),
                None,
                Some(json!({
                    "hint": "Include 3+ lines of context before and after the target text",
                    "matches_found": count
                })),
            ));
        }

        // Generate preview diff
        let new_content = content.replacen(old_string, new_string, 1);
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

        // If no confirmation provided, return diff for review
        if params.confirm.is_none() {
            let mut metadata = HashMap::new();
            metadata.insert("requires_confirmation".to_string(), json!(true));
            metadata.insert("diff".to_string(), json!(diff_output.trim()));
            metadata.insert("added_lines".to_string(), json!(added));
            metadata.insert("removed_lines".to_string(), json!(removed));
            metadata.insert("path".to_string(), json!(path));
            metadata.insert("old_string".to_string(), json!(old_string));
            metadata.insert("new_string".to_string(), json!(new_string));

            return Ok(ToolResult {
                output: format!("Changes require confirmation:\n\n{}", diff_output.trim()),
                success: true,
                metadata,
            });
        }

        // Handle confirmation
        if params.confirm == Some(true) {
            let start = Instant::now();
            
            // Calculate line range affected
            let lines_before = old_string.lines().count() as u32;
            let start_line = content[..content.find(old_string).unwrap_or(0)]
                .lines().count() as u32 + 1;
            let end_line = start_line + lines_before.saturating_sub(1);
            
            // Write the file
            fs::write(path, &new_content).await?;
            
            let duration = start.elapsed();
            
            // Record telemetry
            let file_change = FileChange::modify_with_diff(
                path,
                old_string,
                new_string,
                &diff_output,
                Some((start_line, end_line)),
            );
            
            let mut exec = ToolExecution::start("confirm_edit", json!({
                "path": path,
                "old_string": old_string,
                "new_string": new_string,
            }));
            exec.add_file_change(file_change);
            let exec = exec.complete_success(
                format!("Applied {} changes (+{} -{}) to {}", 
                    added + removed, added, removed, path),
                duration,
            );
            TOOL_EXECUTIONS.record(exec.clone());
            record_persistent(exec);

            Ok(ToolResult::success(format!(
                "✓ Changes applied to {}\n\nDiff:\n{}",
                path,
                diff_output.trim()
            )))
        } else {
            Ok(ToolResult::success("✗ Changes rejected by user"))
        }
    }
}
