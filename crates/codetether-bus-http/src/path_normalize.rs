//! Resolve relative local paths without canonicalization.
use super::*;

pub(super) fn normalize_local_path(path: &Path) -> anyhow::Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    Ok(std::env::current_dir()?.join(path))
}
