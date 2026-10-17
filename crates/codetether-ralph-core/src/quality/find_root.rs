//! Deterministic depth-limited search for a Rust project root.
use std::path::{Path, PathBuf};
use tracing::warn;

/// Find the directory containing Cargo.toml by walking down from the given directory.
/// Returns the original directory if no Cargo.toml is found.
///
/// This handles monorepo setups where the worktree root isn't the crate root.
///
/// # Arguments
/// * `dir` - Root directory to inspect before two sorted descendant levels.
/// # Returns
/// The first directory containing Cargo.toml, or the original directory.
/// # Examples
/// ```no_run
/// let root = codetether_ralph_core::quality::find_cargo_root(std::path::Path::new("."));
/// let _ = root;
/// ```
pub fn find_cargo_root(dir: &Path) -> PathBuf {
    fn has_cargo_toml(path: &Path) -> bool {
        path.join("Cargo.toml").exists()
    }

    // Check current directory first
    if has_cargo_toml(dir) {
        return dir.to_path_buf();
    }

    // Walk down into subdirectories (limited depth)
    if let Ok(entries) = std::fs::read_dir(dir) {
        let mut subdirs: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .map(|e| e.path())
            .collect();

        // Sort for deterministic behavior
        subdirs.sort();

        for subdir in subdirs {
            if has_cargo_toml(&subdir) {
                return subdir;
            }

            // Check one level deeper
            if let Ok(sub_entries) = std::fs::read_dir(&subdir) {
                let mut sub_subdirs: Vec<_> = sub_entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .map(|e| e.path())
                    .collect();

                sub_subdirs.sort();

                for sub_subdir in sub_subdirs {
                    if has_cargo_toml(&sub_subdir) {
                        return sub_subdir;
                    }
                }
            }
        }
    }

    // Fall back to original directory
    warn!(
        dir = %dir.display(),
        "No Cargo.toml found, using worktree root for cargo commands"
    );
    dir.to_path_buf()
}
