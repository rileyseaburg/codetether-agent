//! Detect workspace-wide claims, including lexical and filesystem aliases.
//!
//! The session gate skips these coarse claims; the mux authority rejects them.

use std::path::{Component, Path};

/// Whether a path would claim the workspace rather than a bounded subtree.
///
/// # Arguments
/// * `workspace` — Checkout whose files are being coordinated.
/// * `path` — Relative or absolute requested mutation path.
///
/// # Returns
/// `true` for the root, unbounded directory claims, or aliases resolving to it.
///
/// # Examples
/// For workspace `/repo`, paths `""`, `"."`, and `/repo` are workspace claims;
/// `src/auth.rs` and `src/auth` are bounded claims, unless they alias `/repo`.
pub(crate) fn workspace_claim(workspace: &Path, path: &Path) -> bool {
    if !path
        .components()
        .any(|part| matches!(part, Component::Normal(_)))
    {
        return true;
    }
    let candidate = workspace.join(path);
    if candidate == workspace {
        return true;
    }
    match (
        std::fs::canonicalize(workspace),
        std::fs::canonicalize(candidate),
    ) {
        (Ok(root), Ok(candidate)) => root == candidate,
        _ => false,
    }
}
