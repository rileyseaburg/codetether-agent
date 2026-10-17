//! Worktree path policy for sub-agent tool calls.

use anyhow::Result;
use serde_json::Value;
use std::path::Path;

mod authoritative;
mod batch;
mod inspect;
mod normalize;
mod spec;

/// Normalize declared tool path arguments and reject undeclared path fields.
///
/// # Arguments
/// * `tool` - Tool name used to select its path schema.
/// * `args` - JSON arguments normalized in place.
/// * `root` - Workspace root used for defaults and containment checks.
///
/// # Returns
/// Success after the arguments satisfy the existing workspace path policy.
///
/// # Errors
/// Returns an error for invalid batch calls, undeclared path fields or paths
/// escaping the workspace lexically or through existing symlinks.
///
/// # Examples
/// ```no_run
/// let mut args = serde_json::json!({"path": "src/lib.rs"});
/// let root = std::path::Path::new("/workspace");
/// codetether_tool_core::path_guard::normalize_tool_args("read", &mut args, root).unwrap();
/// assert_eq!(args["path"], "/workspace/src/lib.rs");
/// ```
pub fn normalize_tool_args(tool: &str, args: &mut Value, root: &Path) -> Result<()> {
    if tool == "batch" {
        return batch::normalize(args, root);
    }
    authoritative::apply(tool, args, root);
    let specs = spec::field_specs(tool, args)?;
    for field in &specs {
        normalize::field(args, field, root)?;
    }
    inspect::reject_unknown_path_fields(args, &specs, String::new())?;
    Ok(())
}

#[cfg(test)]
#[path = "authoritative_tests.rs"]
mod authoritative_tests;
#[cfg(test)]
#[path = "batch_tests.rs"]
mod batch_tests;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
