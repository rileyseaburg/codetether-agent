//! Workspace path guard for patch file targets.

use anyhow::{Result, bail};
use std::path::{Component, Path, PathBuf};

pub(super) fn resolve(root: &Path, file: &str) -> Result<PathBuf> {
    let path = Path::new(file);
    if path.is_absolute() {
        bail!("Patch target must be relative: {file}");
    }
    for component in path.components() {
        if matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        ) {
            bail!("Patch target escapes workspace: {file}");
        }
    }
    Ok(root.join(path))
}

#[cfg(test)]
mod tests {
    use super::resolve;
    use std::path::Path;

    #[test]
    fn rejects_absolute_and_parent_paths() {
        assert!(resolve(Path::new("/repo"), "/tmp/x").is_err());
        assert!(resolve(Path::new("/repo"), "../x").is_err());
    }
}
