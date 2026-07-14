//! Path normalization and worktree boundary checks.
//!
//! This module converts user-provided paths into normalized paths rooted in a
//! worktree and rejects paths that would escape that worktree, either
//! lexically (for example through `..`) or through filesystem resolution such
//! as symlinks.

use super::clean;
use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

/// Normalizes a user-provided path and ensures it remains inside `root`.
///
/// Relative paths are interpreted relative to `root`; absolute paths are used
/// as provided. The resulting candidate path is then lexically cleaned and
/// checked to ensure it still starts with `root`. If the normalized path exists,
/// its canonical form is also checked so symlinks cannot resolve outside the
/// worktree.
///
/// # Parameters
///
/// * `raw` - The path supplied by the caller. May be absolute or relative.
/// * `root` - The worktree root that the returned path must stay within.
///
/// # Returns
///
/// Returns the cleaned, normalized path on success.
///
/// # Errors
///
/// Returns an error if the lexical path escapes `root`, or if the existing
/// filesystem path resolves outside `root` after canonicalization.
///
/// # Preconditions
///
/// `root` should identify the intended worktree root. For the lexical
/// `starts_with` check to be meaningful, callers should pass `root` in the same
/// path form expected for normalized candidates, typically an absolute path.
pub fn normalize(raw: &str, root: &Path) -> Result<PathBuf> {
    let normalized = clean::lexical(&candidate(raw, root));
    if !normalized.starts_with(root) {
        bail!("path escapes worktree: {raw}");
    }
    enforce_realpath(raw, root, &normalized)?;
    Ok(normalized)
}

/// Builds the initial path to normalize from a raw user path.
///
/// Absolute inputs are returned directly as path buffers. Relative inputs are
/// joined to the provided worktree `root`.
fn candidate(raw: &str, root: &Path) -> PathBuf {
    if Path::new(raw).is_absolute() {
        PathBuf::from(raw)
    } else {
        root.join(raw)
    }
}

/// Verifies that an existing normalized path does not resolve outside `root`.
///
/// This performs a canonicalization-based check in addition to lexical
/// normalization, which helps prevent symlinks inside the worktree from
/// pointing callers at files outside it. Missing or otherwise non-canonicalizable
/// normalized paths are accepted here; only successfully resolved paths are
/// rejected when they fall outside the canonical root.
///
/// # Errors
///
/// Returns an error when `normalized` exists and canonicalizes to a path outside
/// the canonicalized `root`.
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