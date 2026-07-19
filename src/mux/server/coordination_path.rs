//! Canonical workspace and relative lease-path validation.

use anyhow::{Context, Result, bail};
use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

pub(super) fn workspace(path: &Path) -> Result<PathBuf> {
    std::fs::canonicalize(path).context("canonicalize coordinated workspace")
}

pub(super) fn relative_all(root: &Path, paths: Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    if paths.is_empty() {
        bail!("at least one lease path is required");
    }
    paths
        .into_iter()
        .map(|path| relative(root, &path))
        .collect::<Result<BTreeSet<_>>>()
        .map(|paths| paths.into_iter().collect())
}

fn relative(root: &Path, requested: &Path) -> Result<PathBuf> {
    let candidate = if requested.is_absolute() {
        requested.into()
    } else {
        root.join(requested)
    };
    let relative = candidate
        .strip_prefix(root)
        .context("lease path is outside the mux workspace")?;
    if relative.components().any(|part| {
        matches!(
            part,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        bail!("lease path cannot traverse outside the mux workspace");
    }
    if candidate.exists() && !std::fs::canonicalize(&candidate)?.starts_with(root) {
        bail!("lease path resolves outside the mux workspace");
    }
    Ok(relative.to_path_buf())
}
