//! Project belief save operation.
use super::MEMORY_FILE;
use crate::beliefs::Belief;
use std::path::Path;

/// Save project beliefs to the codebase root.
///
/// # Arguments
/// * `project_root` - Project directory in which to create .codetether/memory.json.
/// * `beliefs` - Records to serialize in their existing order.
///
/// # Returns
/// Unit on success.
///
/// # Errors
/// Returns an error if directory creation, JSON serialization or file writing fails.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_core::palace::save_project_beliefs;
/// save_project_beliefs(std::path::Path::new("."), &[]).unwrap();
/// ```
pub fn save_project_beliefs(project_root: &Path, beliefs: &[Belief]) -> anyhow::Result<()> {
    let dir = project_root.join(".codetether");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    let path = project_root.join(MEMORY_FILE);
    let content = serde_json::to_string_pretty(beliefs)?;
    std::fs::write(path, content)?;
    Ok(())
}
