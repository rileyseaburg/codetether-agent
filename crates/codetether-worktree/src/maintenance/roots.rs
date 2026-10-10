use super::record::WorktreeRecord;
use std::path::{Path, PathBuf};

pub(super) fn normalize(repo: &Path, roots: &[PathBuf]) -> Vec<PathBuf> {
    roots.iter().map(|root| absolute(repo, root)).collect()
}

pub(super) fn includes(record: &WorktreeRecord, roots: &[PathBuf]) -> bool {
    let path = record
        .path
        .canonicalize()
        .unwrap_or_else(|_| record.path.clone());
    roots.is_empty() || roots.iter().any(|root| path.starts_with(root))
}

fn absolute(repo: &Path, root: &Path) -> PathBuf {
    let path = if root.is_absolute() {
        root.into()
    } else {
        repo.join(root)
    };
    path.canonicalize().unwrap_or(path)
}
