//! Format discovered skills or the empty-directory help message.
use super::SkillTool;
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl SkillTool {
    pub(super) async fn list_result(&self) -> Result<ToolResult> {
        let skills = self.list_skills().await?;
        if skills.is_empty() {
            Ok(ToolResult::success(format!(
                "No skills found. Create skills in: {}\n\
                        \n\
                        Skill format:\n\
                        - Directory: ~/.codetether/skills/<skill-name>/SKILL.md\n\
                        - File: ~/.codetether/skills/<skill-name>.md\n\
                        \n\
                        A skill file contains markdown instructions the agent follows.",
                self.skills_dir.display()
            )))
        } else {
            Ok(ToolResult::success(format!(
                "Available skills ({}):\n{}",
                skills.len(),
                skills
                    .iter()
                    .map(|s| format!("  - {}", s))
                    .collect::<Vec<_>>()
                    .join("\n")
            )))
        }
    }
}
