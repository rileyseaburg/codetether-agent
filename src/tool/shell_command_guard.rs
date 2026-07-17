//! Non-bypassable command rules shared by shell execution tools.

use super::ToolResult;

#[path = "shell_worktree_add.rs"]
mod worktree_add;

pub(crate) fn result(tool: &str, command: &str) -> Option<ToolResult> {
    if let Some(mut blocked) = super::bash_file_edit_guard::file_edit_guard_result(command) {
        blocked
            .metadata
            .insert("tool".into(), serde_json::json!(tool));
        return Some(blocked);
    }
    worktree_add::detected(command).then(|| {
        ToolResult::structured_error(
            "DIRECT_WORKTREE_ADD_BLOCKED",
            tool,
            "Direct `git worktree add` is blocked; use CodeTether-managed worktree isolation.",
            None,
            Some(serde_json::json!({
                "required_root": "<workspace-root>/.codetether-worktrees/"
            })),
        )
    })
}

#[cfg(test)]
#[path = "shell_command_guard_tests.rs"]
mod tests;
