use super::clean;
use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

pub fn normalize(raw: &str, root: &Path) -> Result<PathBuf> {
    let normalized = clean::lexical(&candidate(raw, root));
    if !normalized.starts_with(root) {
        bail!("path escapes worktree: {raw}");
    }
    enforce_realpath(raw, root, &normalized)?;
    Ok(normalized)
}

fn candidate(raw: &str, root: &Path) -> PathBuf {
    if Path::new(raw).is_absolute() {
        PathBuf::from(raw)
    } else {
        root.join(raw)
    }
}

fn enforce_realpath(raw: &str, root: &Path, normalized: &Path) -> Result<()> {
    let real_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    if normalized
        .canonicalize()
        .is_ok_and(|real| !real.starts_with(real_root))
    {
        bail!("path resolves outside worktree: {raw}");
    }
    Ok(())
}
