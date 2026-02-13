//! Skill tool: Load and invoke learned skill patterns

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::PathBuf;

/// Skill Tool - Load and execute skill patterns from skill files
/// Skills are markdown files in ~/.codetether/skills/ that contain instructions
/// for specific tasks like "code review", "testing", etc.
pub struct SkillTool {
    skills_dir: PathBuf,
    #[allow(dead_code)]
    cache: HashMap<String, String>,
}

impl Default for SkillTool {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillTool {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        Self {
            skills_dir: PathBuf::from(home).join(".codetether").join("skills"),
            cache: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_dir(dir: PathBuf) -> Self {
        Self {
            skills_dir: dir,
            cache: HashMap::new(),
        }
    }

    /// Get the skill cache for internal tracking
    pub fn cache(&self) -> &HashMap<String, String> {
        &self.cache
    }

    async fn list_skills(&self) -> Result<Vec<String>> {
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
            } else if path.extension().is_some_and(|e| e == "md") {
                if let Some(stem) = path.file_stem().and_then(|n| n.to_str()) {
                    skills.push(stem.to_string());
                }
            }
        }

        Ok(skills)
    }

    async fn load_skill(&self, name: &str) -> Result<String> {
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
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "Action to perform: 'list' (show available skills) or 'load' (load a skill)",
                    "enum": ["list", "load"]
                },
                "skill_name": {
                    "type": "string",
                    "description": "Name of the skill to load (required for 'load' action)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let action = args["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("action is required"))?;

        match action {
            "list" => {
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
            "load" => {
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
            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Use 'list' or 'load'.",
                action
            ))),
        }
    }
}
