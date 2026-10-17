//! Non-bypassable command rules shared by shell execution tools.

use codetether_tool_core::ToolResult;

mod args;
mod python;
mod worktree;
pub use args::result_for_args;

#[path = "shell_git_hooks.rs"]
mod git_hooks;
#[path = "shell_temp_write.rs"]
mod temp_write;
#[path = "shell_worktree_add.rs"]
mod worktree_add;

/// Return the first mandatory rejection for a shell command.
///
/// # Arguments
/// * `tool` - Tool identifier attached to the rejection.
/// * `command` - Shell command text to inspect.
/// # Returns
/// The first matching rejection, or `None` when no rule matches.
/// # Examples
/// ```rust
/// assert!(codetether_command_guard::result("bash", "echo ready").is_none());
/// ```
pub fn result(tool: &str, command: &str) -> Option<ToolResult> {
    if let Some(blocked) = python::result(tool, command) {
        return Some(blocked);
    }
    if let Some(mut blocked) = file_edit_guard::file_edit_guard_result(command) {
        blocked
            .metadata
            .insert("tool".into(), serde_json::json!(tool));
        return Some(blocked);
    }
    if let Some(path) = temp_write::detected(command) {
        return codetether_tool_core::temp_write_guard::denied_result(tool, &path);
    }
    if git_hooks::detected(command) {
        return Some(ToolResult::structured_error(
            "GIT_HOOK_BYPASS_BLOCKED",
            tool,
            "Commits and pushes must run the repository's hooks; `--no-verify` \
             and hook-path overrides are blocked.",
            None,
            Some(serde_json::json!({
                "fix": "Rerun without the bypass and address hook failures."
            })),
        ));
    }
    worktree::result(tool, command)
}

mod file_edit_guard;
pub use file_edit_guard::{file_edit_guard_reason, file_edit_guard_result};
