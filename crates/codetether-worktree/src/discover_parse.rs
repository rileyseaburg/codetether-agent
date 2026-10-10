use super::WorktreeInfo;
use std::path::{Path, PathBuf};

pub(super) fn parse_worktree_list(output: &str, base_dir: &Path) -> Vec<WorktreeInfo> {
    output
        .split("\n\n")
        .filter_map(|block| parse_block(block, base_dir))
        .collect()
}

fn parse_block(block: &str, base_dir: &Path) -> Option<WorktreeInfo> {
    let mut path = None;
    let mut branch = None;
    for line in block.lines() {
        if let Some(rest) = line.strip_prefix("worktree ") {
            path = Some(PathBuf::from(rest));
        } else if let Some(rest) = line.strip_prefix("branch refs/heads/") {
            branch = Some(rest.to_string());
        }
    }
    let path = path?;
    let branch = branch?;
    let is_codetether = branch.starts_with("codetether/");
    if !is_codetether && !path.starts_with(base_dir) {
        return None;
    }
    let name = branch
        .strip_prefix("codetether/")
        .map(str::to_string)
        .or_else(|| {
            path.file_name()
                .map(|name| name.to_string_lossy().to_string())
        })?;
    Some(WorktreeInfo {
        name,
        path,
        branch,
        active: true,
    })
}
