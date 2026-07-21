//! Rejection of dangerous whole-filesystem and whole-home lease scopes.

use std::path::Path;

pub(super) fn unsafe_workspace(workspace: &Path) -> bool {
    workspace.parent().is_none()
        || directories::BaseDirs::new().is_some_and(|dirs| workspace == dirs.home_dir())
}

pub(super) fn result(tool: &str, workspace: &Path) -> super::super::tool_policy::ToolTuple {
    super::gate_error::result(
        "WORKTREE_SCOPE_REQUIRED",
        tool,
        "Refusing to lease the filesystem or home directory. Set workdir/cwd to the exact repository and retry locally.",
        serde_json::json!({ "workspace": workspace }),
    )
}
