//! Non-bypassable ban on writing into system temporary directories.
//!
//! Models reach for `/tmp` to stage scratch files, evidence, and "quick
//! tests". Those writes leave the workspace, survive worktree cleanup, are
//! invisible to `git status`, and let validation artifacts disappear between
//! runs. The denied roots are compiled into the binary and there is no
//! environment variable or config key to turn this off.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::tool::temp_write_guard::denied_reason;
//!
//! assert!(denied_reason("/tmp/scratch.txt").is_some());
//! assert!(denied_reason("src/main.rs").is_none());
//! ```

use std::path::Path;

#[path = "temp_write_guard_path.rs"]
mod path_check;
#[path = "temp_write_guard_roots.rs"]
mod roots;

/// Every hard-coded temp root, including runtime-discovered locations.
pub(crate) fn denied_roots() -> Vec<std::path::PathBuf> {
    roots::all()
}

/// Returns a human-readable refusal when `path` writes into a temp directory.
///
/// Returns `None` for any path outside the hard-coded temp roots.
pub fn denied_reason(path: &str) -> Option<String> {
    let root = path_check::denied_root(Path::new(path))?;
    Some(format!(
        "Writing to the temporary directory {} is not permitted; \
         write inside the workspace so the change is reviewable and durable",
        root.display()
    ))
}

/// Structured refusal for a tool that was asked to write into a temp path.
pub fn denied_result(tool: &str, path: &str) -> Option<crate::tool::ToolResult> {
    let reason = denied_reason(path)?;
    Some(crate::tool::ToolResult::structured_error(
        "TEMP_DIR_WRITE_BLOCKED",
        tool,
        &reason,
        None,
        Some(serde_json::json!({
            "attempted_path": path,
            "write_instead": "a path inside the workspace, e.g. ./artifacts/<name>",
            "why": "Temp files escape the workspace, evade review, and vanish between runs.",
        })),
    ))
}

#[cfg(test)]
#[path = "temp_write_guard_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "temp_write_guard_boundary_tests.rs"]
mod boundary_tests;
