//! Translate direct-worktree detection into the shared shell rejection result.

pub(super) fn result(tool: &str, command: &str) -> Option<crate::tool::ToolResult> {
    super::worktree_add::detected(command).then(|| {
        crate::tool::ToolResult::structured_error(
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
