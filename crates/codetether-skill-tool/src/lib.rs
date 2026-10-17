//! Skill discovery and Markdown instruction loading.
//!
//! [SkillTool] lists file-based and directory-based skills and loads their
//! instruction text. The default root is ~/.codetether/skills.
//!
//! ```rust
//! let tool = codetether_skill_tool::SkillTool::with_dir("skills".into());
//! assert!(tool.cache().is_empty());
//! ```
use std::{collections::HashMap, path::PathBuf};
mod catalog;
mod list_action;
mod load;
mod load_action;
mod schema;
mod tool;
/// Discover and load reusable Markdown skill instructions.
///
/// # Examples
/// ```rust
/// let _tool = codetether_skill_tool::SkillTool::new();
/// ```
pub struct SkillTool {
    skills_dir: PathBuf,
    #[allow(dead_code)]
    cache: HashMap<String, String>,
}
impl SkillTool {
    /// Construct a tool using the default skill directory.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool rooted at HOME/.codetether/skills, or ./.codetether/skills.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_skill_tool::SkillTool::new();
    /// ```
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        Self {
            skills_dir: PathBuf::from(home).join(".codetether").join("skills"),
            cache: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    /// Construct a tool rooted at an explicit skill directory.
    ///
    /// # Arguments
    /// `dir` is the directory containing Markdown skills or skill subdirectories.
    /// # Returns
    /// A tool using that directory and an empty cache.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_skill_tool::SkillTool::with_dir("skills".into());
    /// ```
    pub fn with_dir(dir: PathBuf) -> Self {
        Self {
            skills_dir: dir,
            cache: HashMap::new(),
        }
    }

    /// Get the skill cache for internal tracking
    #[allow(dead_code)]
    ///
    /// # Examples
    /// ```rust
    /// assert!(codetether_skill_tool::SkillTool::new().cache().is_empty());
    /// ```
    pub fn cache(&self) -> &HashMap<String, String> {
        &self.cache
    }
}

impl Default for SkillTool {
    fn default() -> Self {
        Self::new()
    }
}
