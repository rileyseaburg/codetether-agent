//! Load directory-based skills before flat Markdown files.
use super::SkillTool;
use anyhow::Result;
impl SkillTool {
    pub(super) async fn load_skill(&self, name: &str) -> Result<String> {
        // Check for directory-based skill first
        let dir_skill = self.skills_dir.join(name).join("SKILL.md");
        if dir_skill.exists() {
            return Ok(tokio::fs::read_to_string(&dir_skill).await?);
        }

        // Check for file-based skill
        let file_skill = self.skills_dir.join(format!("{}.md", name));
        if file_skill.exists() {
            return Ok(tokio::fs::read_to_string(&file_skill).await?);
        }

        anyhow::bail!("Skill '{}' not found", name)
    }
}
