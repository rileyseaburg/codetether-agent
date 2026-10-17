//! Enumerate supported skill files and directories.
use super::SkillTool;
use anyhow::Result;
impl SkillTool {
    pub(super) async fn list_skills(&self) -> Result<Vec<String>> {
        let mut skills = Vec::new();

        if !self.skills_dir.exists() {
            return Ok(skills);
        }

        let mut entries = tokio::fs::read_dir(&self.skills_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Check if SKILL.md exists in the directory
                    let skill_md = path.join("SKILL.md");
                    if skill_md.exists() {
                        skills.push(name.to_string());
                    }
                }
            } else if path.extension().is_some_and(|e| e == "md")
                && let Some(stem) = path.file_stem().and_then(|n| n.to_str())
            {
                skills.push(stem.to_string());
            }
        }

        Ok(skills)
    }
}
