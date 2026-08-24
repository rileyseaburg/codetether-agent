//! Writable checkout and Git metadata paths for sandboxed mutations.

use std::path::{Path, PathBuf};

pub(super) fn writable(cwd: &Path) -> Vec<PathBuf> {
    let mut paths = vec![cwd.to_path_buf()];
    let dot_git = cwd.join(".git");
    if dot_git.is_dir() {
        paths.push(dot_git);
    } else if let Ok(content) = std::fs::read_to_string(&dot_git)
        && let Some(value) = content.trim().strip_prefix("gitdir:")
    {
        let requested = Path::new(value.trim());
        let joined = if requested.is_absolute() {
            requested.to_path_buf()
        } else {
            cwd.join(requested)
        };
        if let Ok(path) = joined.canonicalize()
            && trusted(cwd, &path)
        {
            paths.push(root(&path).unwrap_or(path));
        }
    }
    paths
}

fn trusted(cwd: &Path, path: &Path) -> bool {
    cwd.ancestors()
        .any(|parent| path.starts_with(parent.join(".git")))
}

fn root(path: &Path) -> Option<PathBuf> {
    path.ancestors()
        .find(|part| part.file_name().is_some_and(|name| name == ".git"))
        .map(Path::to_path_buf)
}

#[cfg(test)]
#[test]
fn linked_worktree_allows_its_common_git_metadata() {
    let root = tempfile::tempdir().expect("root");
    let checkout = root.path().join("worktree");
    let metadata = root.path().join(".git/worktrees/worktree");
    std::fs::create_dir_all(&checkout).expect("checkout");
    std::fs::create_dir_all(&metadata).expect("metadata");
    std::fs::write(checkout.join(".git"), "gitdir: ../.git/worktrees/worktree").expect("link");
    let paths = writable(&checkout.canonicalize().expect("canonical checkout"));
    assert!(paths.contains(&root.path().join(".git").canonicalize().expect("git root")));
}
