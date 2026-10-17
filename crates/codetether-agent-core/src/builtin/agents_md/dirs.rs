//! Directory ordering for AGENTS.md discovery.

use std::path::{Path, PathBuf};

pub(super) fn root_to_leaf(start_dir: &Path) -> Vec<PathBuf> {
    let mut dirs = leaf_to_root(start_dir);
    dirs.reverse();
    dirs
}

fn leaf_to_root(start_dir: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let mut current = start_dir.to_path_buf();
    let repo_root = find_git_root(start_dir);
    loop {
        dirs.push(current.clone());
        if repo_root.as_ref() == Some(&current) || !current.pop() {
            break;
        }
    }
    dirs
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
