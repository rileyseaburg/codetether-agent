//! Project belief load operation.
use super::MEMORY_FILE;
use crate::beliefs::Belief;
use std::path::Path;

/// Load project beliefs from the codebase root.
///
/// # Arguments
/// * `project_root` - Directory containing the project’s .codetether memory folder.
///
/// # Returns
/// Persisted beliefs, or an empty vector for missing, unreadable or invalid files.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_core::palace::load_project_beliefs;
/// let beliefs = load_project_beliefs(std::path::Path::new("."));
/// assert!(beliefs.iter().all(|b| !b.id.is_empty()));
/// ```
pub fn load_project_beliefs(project_root: &Path) -> Vec<Belief> {
    let path = project_root.join(MEMORY_FILE);
    if !path.exists() {
        return Vec::new();
    }
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}
