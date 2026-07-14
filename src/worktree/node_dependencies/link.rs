use super::{git, symlink};
use anyhow::{Result, bail};
use std::{fs, path::Path};

pub(super) fn package(source: &Path, worktree: &Path, relative: &Path) -> Result<bool> {
    let package = worktree.join(relative);
    if !package.join("package.json").is_file() {
        return Ok(false);
    }
    let package_root = package.canonicalize()?;
    if !package_root.starts_with(worktree) {
        bail!("package path escapes worktree: {}", relative.display());
    }
    let source_modules = source.join(relative).join("node_modules");
    if !source_modules.is_dir() {
        return Ok(false);
    }
    let destination = package.join("node_modules");
    git::require_ignored(worktree, &relative.join("node_modules"))?;
    if let Ok(metadata) = fs::symlink_metadata(&destination) {
        if metadata.file_type().is_symlink()
            && fs::canonicalize(&destination)? == fs::canonicalize(&source_modules)?
        {
            return Ok(false);
        }
        bail!("refusing to replace existing {}", destination.display());
    }
    symlink::create(&source_modules.canonicalize()?, &destination)?;
    Ok(true)
}
