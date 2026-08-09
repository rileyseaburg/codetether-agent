use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use super::super::{Tool, ToolResult};
use super::{execute, schema};

/// Edit files by replacing strings.
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
        "edit(path, old_string, new_string, replace_all?) - Replace text. Falls back to whitespace-tolerant and fuzzy nearest-block matching."
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute::run(args).await
    }
}
