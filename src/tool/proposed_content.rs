//! Proposed file contents for any approval-gated file mutation.
//!
//! Every write-capable tool (`apply_patch`, `write`, `edit`, `multiedit`)
//! must yield the same `(path, content)` shape so the approval overlay can
//! run real language-server diagnostics before the user decides.

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
pub(crate) use codetether_tool_core::proposed::supported;
pub(super) use codetether_tool_core::proposed::{resolve, text};
use serde_json::Value;

/// Reconstruct the post-mutation content of every file a tool call touches.
///
/// # Errors
///
/// Returns an error when the tool is not a file mutation, the arguments are
/// malformed, or an edit target cannot be located in the current file.
pub(crate) fn contents(root: &Path, tool: &str, args: &Value) -> Result<Vec<(PathBuf, String)>> {
    match tool {
        "apply_patch" | "patch" => {
            let patch = text(args, "patch")?;
            crate::tool::patch::proposed::contents(root, patch)
        }
        "write" => {
            let path = resolve(root, text(args, "path")?);
            Ok(vec![(path, text(args, "content")?.to_string())])
        }
        "edit" => super::edit::proposed::single(root, args),
        "multiedit" => super::edit::proposed::many(root, args),
        other => bail!("{other} does not mutate files"),
    }
}

#[cfg(test)]
#[path = "proposed_content_tests.rs"]
mod tests;
