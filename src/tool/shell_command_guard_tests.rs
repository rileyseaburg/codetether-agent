use super::result;
use crate::tool::{Tool, bash::BashTool, exec_command::ExecCommandTool};
use std::sync::Arc;

#[test]
fn blocks_direct_worktree_creation_forms() {
    for command in [
        "git worktree add /tmp/random",
        "/usr/bin/git -C repo worktree add ../sibling",
        "sudo -n git worktree add .codetether-worktrees/task",
    ] {
        let blocked = result("bash", command).expect("direct worktree add must be blocked");
        assert!(!blocked.success, "{command}");
        assert!(blocked.output.contains("DIRECT_WORKTREE_ADD_BLOCKED"));
    }
}

#[test]
fn allows_managed_and_read_only_worktree_commands() {
    assert!(result("bash", "git worktree list").is_none());
    assert!(result("bash", "rg 'git worktree add' docs").is_none());
}

#[tokio::test]
async fn bash_tool_enforces_worktree_guard() {
    let blocked = BashTool::new()
        .execute(serde_json::json!({"command": "git worktree add /tmp/random"}))
        .await
        .expect("guard result");
    assert!(!blocked.success);
    assert!(blocked.output.contains("DIRECT_WORKTREE_ADD_BLOCKED"));
}

#[tokio::test]
async fn exec_command_tool_enforces_worktree_guard() {
    let sessions = Arc::new(crate::tool::command_session::Registry::default());
    let tool = ExecCommandTool::new(sessions, None);
    let blocked = tool
        .execute(serde_json::json!({"cmd": "git worktree add /tmp/random"}))
        .await
        .expect("guard result");
    assert!(!blocked.success);
    assert!(blocked.output.contains("DIRECT_WORKTREE_ADD_BLOCKED"));
}
