//! Edit tool: replace strings in files

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use similar::{ChangeTag, TextDiff};
use tokio::fs;

/// Edit files by replacing strings
pub struct EditTool;

impl Default for EditTool {
    fn default() -> Self {
        Self::new()
    }
}

impl EditTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for EditTool {
    fn id(&self) -> &str {
        "edit"
    }

    fn name(&self) -> &str {
        "Edit File"
    }

    fn description(&self) -> &str {
        "edit(path: string, old_string: string, new_string: string) - Edit a file by replacing an exact string with new content. Include enough context (3+ lines before and after) to uniquely identify the location."
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
                    "description": "The exact string to replace (must match exactly, including whitespace)"
                },
                "new_string": {
                    "type": "string",
                    "description": "The string to replace old_string with"
                }
            },
            "required": ["path", "old_string", "new_string"],
            "example": {
                "path": "src/main.rs",
                "old_string": "fn old_function() {\n    println!(\"old\");\n}",
                "new_string": "fn new_function() {\n    println!(\"new\");\n}"
            }
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let path = match args["path"].as_str() {
            Some(p) => p,
            None => {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "edit",
                    "path is required",
                    Some(vec!["path"]),
                    Some(
                        json!({"path": "src/main.rs", "old_string": "old text", "new_string": "new text"}),
                    ),
                ));
            }
        };
        let old_string = match args["old_string"].as_str() {
            Some(s) => s,
            None => {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "edit",
                    "old_string is required",
                    Some(vec!["old_string"]),
                    Some(json!({"path": path, "old_string": "old text", "new_string": "new text"})),
                ));
            }
        };
        let new_string = match args["new_string"].as_str() {
            Some(s) => s,
            None => {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "edit",
                    "new_string is required",
                    Some(vec!["new_string"]),
                    Some(json!({"path": path, "old_string": old_string, "new_string": "new text"})),
                ));
            }
        };

        // Read the file
        let content = fs::read_to_string(path).await?;

        // Count occurrences
        let count = content.matches(old_string).count();

        if count == 0 {
            return Ok(ToolResult::structured_error(
                "NOT_FOUND",
                "edit",
                "old_string not found in file. Make sure it matches exactly, including whitespace.",
                None,
                Some(json!({
                    "hint": "Use the 'read' tool first to see the exact content of the file",
                    "path": path,
                    "old_string": "<copy exact text from file including whitespace>",
                    "new_string": "replacement text"
                })),
            ));
        }

        if count > 1 {
            return Ok(ToolResult::structured_error(
                "AMBIGUOUS_MATCH",
                "edit",
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
                diff_output.push_str(line.trim_end());
            }
            diff_output.push('\n');
        }

        // Instead of applying changes immediately, return confirmation prompt
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("requires_confirmation".to_string(), serde_json::json!(true));
        metadata.insert("diff".to_string(), serde_json::json!(diff_output.trim()));
        metadata.insert("added_lines".to_string(), serde_json::json!(added));
        metadata.insert("removed_lines".to_string(), serde_json::json!(removed));
        metadata.insert("path".to_string(), serde_json::json!(path));
        metadata.insert("old_string".to_string(), serde_json::json!(old_string));
        metadata.insert("new_string".to_string(), serde_json::json!(new_string));

        Ok(ToolResult {
            output: format!("Changes require confirmation:\n\n{}", diff_output.trim()),
            success: true,
            metadata,
        })
    }
}
