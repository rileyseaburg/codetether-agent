//! Shared Git repository root discovery for mutation targets.

use std::path::{Path, PathBuf};

pub(super) fn shared(paths: &[PathBuf]) -> Option<PathBuf> {
    let roots = paths
        .iter()
        .map(|path| project(path))
        .collect::<Option<Vec<_>>>()?;
    let first = roots.first()?.clone();
    roots.iter().all(|root| *root == first).then_some(first)
}

pub(super) fn single_directory(paths: &[PathBuf]) -> Option<PathBuf> {
    (paths.len() == 1 && paths[0].is_dir()).then(|| paths[0].clone())
}

fn project(path: &Path) -> Option<PathBuf> {
    let start = if path.is_dir() { path } else { path.parent()? };
    start
        .ancestors()
        .find(|candidate| candidate.join(".git").exists())
        .map(Path::to_path_buf)
}
