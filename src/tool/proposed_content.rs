//! Proposed file contents for any approval-gated file mutation.
//!
//! Every write-capable tool (`apply_patch`, `write`, `edit`, `multiedit`)
//! must yield the same `(path, content)` shape so the approval overlay can
//! run real language-server diagnostics before the user decides.

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use serde_json::Value;

/// Whether [`contents`] can reconstruct proposed files for this tool.
pub(crate) fn supported(tool: &str) -> bool {
    matches!(
        tool,
        "apply_patch" | "patch" | "write" | "edit" | "multiedit"
    )
}

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

pub(super) fn resolve(root: &Path, path: &str) -> PathBuf {
    let path = Path::new(path);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

pub(super) fn text<'a>(args: &'a Value, key: &str) -> Result<&'a str> {
    args.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("missing `{key}` argument"))
}

#[cfg(test)]
#[path = "proposed_content_tests.rs"]
mod tests;
