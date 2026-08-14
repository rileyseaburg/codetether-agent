//! Stage a generated Seatbelt profile on disk for `sandbox-exec -f`.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[path = "sandbox_seatbelt_prune.rs"]
mod prune;

/// Write `profile` to a uniquely named file inside `temp_dir`.
///
/// Stale profiles from earlier runs are pruned first so the temp directory
/// does not grow without bound.
///
/// # Errors
///
/// Returns an error when the file cannot be created or written.
pub(super) fn write(profile: &str, temp_dir: &Path) -> Result<PathBuf> {
    prune::stale(temp_dir, std::time::SystemTime::now());
    let path = temp_dir.join(file_name());
    std::fs::write(&path, profile)
        .with_context(|| format!("Failed to write Seatbelt profile: {}", path.display()))?;
    Ok(path)
}

fn file_name() -> String {
    format!("codetether-seatbelt-{}.sb", uuid::Uuid::new_v4())
}

#[cfg(test)]
#[path = "sandbox_seatbelt_stage_tests.rs"]
mod tests;
