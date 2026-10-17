//! Validate the requested name and return its instruction text.
use super::SkillTool;
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;
impl SkillTool {
    pub(super) async fn load_result(&self, args: &Value) -> Result<ToolResult> {
        let name = args["skill_name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("skill_name is required for 'load' action"))?;

        match self.load_skill(name).await {
            Ok(content) => Ok(ToolResult::success(format!(
                "=== Skill: {} ===\n\n{}",
                name, content
            ))),
            Err(e) => Ok(ToolResult::error(format!("Failed to load skill: {}", e))),
        }
    }
}
