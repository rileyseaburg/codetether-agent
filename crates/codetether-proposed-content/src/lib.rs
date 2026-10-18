//! Proposed file contents for any approval-gated file mutation.
//!
//! Every write-capable tool (`apply_patch`, `write`, `edit`, `multiedit`)
//! must yield the same `(path, content)` shape so the approval overlay can
//! run real language-server diagnostics before the user decides.

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
pub use codetether_tool_core::proposed::supported;
use codetether_tool_core::proposed::{resolve, text};
use serde_json::Value;

/// Reconstruct the post-mutation content of every file a tool call touches.
///
/// # Arguments
/// * `root` - Workspace directory used to resolve target paths.
/// * `tool` - Tool name: apply_patch, patch, write, edit or multiedit.
/// * `args` - Original tool arguments.
/// # Returns
/// Target paths and proposed complete file contents without writing them.
/// # Examples
/// ```
/// let files = codetether_proposed_content::contents(std::path::Path::new("."), "write",
///     &serde_json::json!({"path":"note.txt", "content":"hello"})).unwrap();
/// assert_eq!(files[0].1, "hello");
/// ```
/// # Errors
///
/// Returns an error when the tool is not a file mutation, the arguments are
/// malformed, or an edit target cannot be located in the current file.
pub fn contents(root: &Path, tool: &str, args: &Value) -> Result<Vec<(PathBuf, String)>> {
    match tool {
        "apply_patch" | "patch" => {
            let patch = text(args, "patch")?;
            codetether_patch::proposed::contents(root, patch)
        }
        "write" => {
            let path = resolve(root, text(args, "path")?);
            Ok(vec![(path, text(args, "content")?.to_string())])
        }
        "edit" => codetether_edit::proposed::single(root, args),
        "multiedit" => codetether_edit::proposed::many(root, args),
        other => bail!("{other} does not mutate files"),
    }
}

#[cfg(test)]
mod edit_applies_replacement_without_writing;
#[cfg(test)]
mod multiedit_chains_edits_to_the_same_file;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod unsupported_tools_are_rejected;
#[cfg(test)]
mod write_uses_full_content;
