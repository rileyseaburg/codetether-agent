//! AGENTS.md discovery helpers for built-in prompts.
//!
//! This module searches for project instruction files while respecting the git
//! repository boundary.
//!
//! # Examples
//!
//! ```ignore
//! let all = load_all_agents_md(std::path::Path::new("."));
//! ```

use std::path::{Path, PathBuf};

/// Loads the closest `AGENTS.md` at or above `start_dir`, stopping at the git root.
///
/// # Examples
///
/// ```ignore
/// let loaded = load_agents_md(std::path::Path::new("."));
/// ```
pub fn load_agents_md(start_dir: &Path) -> Option<(String, PathBuf)> {
    load_all_agents_md(start_dir).into_iter().next()
}

/// Loads all `AGENTS.md` files between `start_dir` and the git root.
///
/// # Examples
///
/// ```ignore
/// let files = load_all_agents_md(std::path::Path::new("."));
/// ```
pub fn load_all_agents_md(start_dir: &Path) -> Vec<(String, PathBuf)> {
    let mut results = Vec::new();
    let mut current = start_dir.to_path_buf();
    let repo_root = find_git_root(start_dir);
    loop {
        if let Some(found) = load_file(&current) {
            results.push(found);
        }
        if repo_root.as_ref() == Some(&current) || !current.pop() {
            break;
        }
    }
    results
}

fn load_file(dir: &Path) -> Option<(String, PathBuf)> {
    let agents_path = dir.join("AGENTS.md");
    std::fs::read_to_string(&agents_path)
        .ok()
        .map(|content| (content, agents_path))
}

fn find_git_root(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();
    loop {
        if current.join(".git").exists() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}
