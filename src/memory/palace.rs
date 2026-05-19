//! Project Memory Palace — persistent beliefs scoped to a codebase root.

use std::path::Path;

use crate::cognition::beliefs::{Belief, BeliefStatus};

/// File name for persisted project beliefs.
const MEMORY_FILE: &str = ".codetether/memory.json";

/// Load project beliefs from the codebase root.
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

/// Save project beliefs to the codebase root.
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

/// Build a system-context string from high-confidence beliefs.
/// Only includes beliefs with confidence >= 0.7 and Active status.
pub fn belief_context(beliefs: &[Belief]) -> String {
    let active: Vec<&Belief> = beliefs
        .iter()
        .filter(|b| b.confidence >= 0.7 && b.status == BeliefStatus::Active)
        .take(20)
        .collect();
    if active.is_empty() {
        return String::new();
    }
    let mut ctx = String::from("Project knowledge:\n");
    for b in &active {
        ctx.push_str(&format!("- {} (conf: {:.2})\n", b.claim, b.confidence));
    }
    ctx
}
