//! Tool registry metadata and list/load action dispatch.
use super::SkillTool;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl Tool for SkillTool {
    fn id(&self) -> &str {
        "skill"
    }

    fn name(&self) -> &str {
        "Skill"
    }

    fn description(&self) -> &str {
        "Load and invoke learned skill patterns. Skills are reusable instruction sets for specific tasks like code review, testing, documentation, etc. Use 'list' action to see available skills, 'load' to read a skill's instructions."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let action = args["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("action is required"))?;

        match action {
            "list" => self.list_result().await,
            "load" => self.load_result(&args).await,
            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Use 'list' or 'load'.",
                action
            ))),
        }
    }
}
