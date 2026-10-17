//! Git-backed scope identity for memory records.

use std::path::{Path, PathBuf};

pub(super) fn remote(cwd: &Path) -> Option<String> {
    git(cwd, ["remote", "get-url", "origin"]).map(|s| format!("git:{s}"))
}

pub(super) fn common(cwd: &Path) -> Option<String> {
    let raw = PathBuf::from(git(cwd, ["rev-parse", "--git-common-dir"])?);
    let path = if raw.is_absolute() {
        raw
    } else {
        cwd.join(raw)
    };
    path_scope("git-common", &path)
}

pub(super) fn path_scope(kind: &str, path: &Path) -> Option<String> {
    path.canonicalize()
        .ok()
        .map(|p| format!("{kind}:{}", p.display()))
}

fn git<const N: usize>(cwd: &Path, args: [&str; N]) -> Option<String> {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}
