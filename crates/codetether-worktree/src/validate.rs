use super::WorktreeManager;
use anyhow::{Result, anyhow};

impl WorktreeManager {
    pub(crate) fn validate_worktree_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("Worktree name cannot be empty"));
        }
        if name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Ok(());
        }
        Err(anyhow!(
            "Invalid worktree name '{}'. Only alphanumeric characters, '-' and '_' are allowed.",
            name
        ))
    }
}
