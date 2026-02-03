//! Edit tool: replace strings in files

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use similar::{ChangeTag, TextDiff};
use tokio::fs;

/// Edit files by replacing strings
pub struct EditTool;

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
            None => return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "edit",
                "path is required",
                Some(vec!["path"]),
                Some(json!({"path": "src/main.rs", "old_string": "old text", "new_string": "new text"})),
            )),
        };
        let old_string = match args["old_string"].as_str() {
            Some(s) => s,
            None => return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "edit",
                "old_string is required",
                Some(vec!["old_string"]),
                Some(json!({"path": path, "old_string": "old text", "new_string": "new text"})),
            )),
        };
        let new_string = match args["new_string"].as_str() {
            Some(s) => s,
            None => return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "edit",
                "new_string is required",
                Some(vec!["new_string"]),
                Some(json!({"path": path, "old_string": old_string, "new_string": "new text"})),
            )),
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
                &format!("old_string found {} times. Include more context to uniquely identify the location.", count),
                None,
                Some(json!({
                    "hint": "Include 3+ lines of context before and after the target text",
                    "matches_found": count
                })),
            ));
        }

        // Perform the replacement
        let new_content = content.replacen(old_string, new_string, 1);

        // Generate diff for output
        let diff = TextDiff::from_lines(&content, &new_content);
        let mut diff_output = String::new();
        
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            diff_output.push_str(&format!("{}{}", sign, change));
        }

        // Write the file
        fs::write(path, &new_content).await?;

        Ok(ToolResult::success(format!("Successfully edited {}\n\n{}", path, diff_output)))
    }
}
