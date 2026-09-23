//! Non-bypassable command rules shared by shell execution tools.

use super::ToolResult;

#[path = "shell_git_hooks.rs"]
mod git_hooks;
#[path = "shell_temp_write.rs"]
mod temp_write;
#[path = "shell_worktree_add.rs"]
mod worktree_add;

pub(crate) fn result(tool: &str, command: &str) -> Option<ToolResult> {
    if let Some(mut blocked) = super::bash_file_edit_guard::file_edit_guard_result(command) {
        blocked
            .metadata
            .insert("tool".into(), serde_json::json!(tool));
        return Some(blocked);
    }
    if let Some(path) = temp_write::detected(command) {
        return super::temp_write_guard::denied_result(tool, &path);
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

/// Guard over a full argument object, reading the tool's command field.
///
/// `cwd`/`workdir` are deliberately **not** treated as temp violations: the
/// sandbox uses [`std::env::temp_dir`] as its own default working directory,
/// so banning a temp cwd would break sandboxed execution and override
/// explicitly approved invocations. Temp *paths* are still refused.
pub(crate) fn result_for_args(tool: &str, args: &serde_json::Value) -> Option<ToolResult> {
    let command = args["command"]
        .as_str()
        .or_else(|| args["cmd"].as_str())
        .unwrap_or_default();
    result(tool, command)
}

#[cfg(test)]
#[path = "shell_command_guard_tests.rs"]
mod tests;
