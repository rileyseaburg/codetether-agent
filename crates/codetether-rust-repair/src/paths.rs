//! Platform-independent snapshot paths and protected edit targets.
use anyhow::{Result, ensure};

/// Validate a relative POSIX path without accessing the filesystem.
/// # Arguments
/// * `path` - A snapshot-relative path.
/// # Returns
/// Unit when every component is a normal relative name.
/// # Errors
/// Rejects empty, absolute, traversal, backslash, colon and control characters.
/// # Examples
/// ```
/// assert!(codetether_rust_repair::validate_path("src/lib.rs").is_ok());
/// assert!(codetether_rust_repair::validate_path("../secret").is_err());
/// ```
pub fn validate_path(path: &str) -> Result<()> {
    ensure!(
        !path.is_empty() && !path.contains(['\\', ':']) && !path.chars().any(char::is_control),
        "invalid relative POSIX path: {path}"
    );
    ensure!(
        path.split('/').all(|part| !matches!(part, "" | "." | "..")),
        "invalid path components: {path}"
    );
    Ok(())
}

pub(crate) fn forbidden(path: &str) -> bool {
    path.split('/').any(|part| {
        let part = part.to_ascii_lowercase();
        part == ".git"
            || part == ".env"
            || part.starts_with(".env.")
            || part == "agents.md"
            || part == "agents.override.md"
            || part == "claude.md"
            || part == "gemini.md"
            || part.ends_with(".instructions.md")
            || part == "copilot-instructions.md"
            || part.ends_with(".lock")
            || part.ends_with("-lock.json")
            || part.ends_with("-lock.yaml")
            || part == "npm-shrinkwrap.json"
    })
}
