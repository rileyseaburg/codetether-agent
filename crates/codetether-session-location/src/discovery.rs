//! Bounded compatibility lookup for sessions saved before location pointers.

use std::path::{Path, PathBuf};

pub(super) fn find(id: &str) -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    cwd.ancestors().find_map(|root| under(root, id))
}

fn under(root: &Path, id: &str) -> Option<PathBuf> {
    let direct = session_file(root, id);
    if direct.is_file() {
        return Some(direct);
    }
    let entries = std::fs::read_dir(root.join(".codetether-worktrees")).ok()?;
    entries.filter_map(Result::ok).find_map(|entry| {
        let candidate = session_file(&entry.path(), id);
        candidate.is_file().then_some(candidate)
    })
}

fn session_file(workspace: &Path, id: &str) -> PathBuf {
    workspace
        .join(".codetether-agent/sessions")
        .join(format!("{id}.json"))
}

#[cfg(test)]
mod tests;
