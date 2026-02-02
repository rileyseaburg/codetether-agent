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
        "Edit a file by replacing an exact string with new content. Include enough context (3+ lines before and after) to uniquely identify the location."
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
            "required": ["path", "old_string", "new_string"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("path is required"))?;
        let old_string = args["old_string"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("old_string is required"))?;
        let new_string = args["new_string"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("new_string is required"))?;

        // Read the file
        let content = fs::read_to_string(path).await?;

        // Count occurrences
        let count = content.matches(old_string).count();

        if count == 0 {
            return Ok(ToolResult::error(
                "old_string not found in file. Make sure it matches exactly, including whitespace.",
            ));
        }

        if count > 1 {
            return Ok(ToolResult::error(format!(
                "old_string found {} times. Include more context to uniquely identify the location.",
                count
            )));
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
