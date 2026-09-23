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

#[test]
fn blocks_every_hook_bypass_form() {
    for command in [
        "git commit --no-verify -m fix",
        "git commit -n -m fix",
        "git commit -anm fix",
        "git -C repo commit -n -m fix",
        "git push --no-verify origin HEAD",
        "cd repo && git push --no-verify",
        "git merge --no-verify main",
        "git -c core.hooksPath=/dev/null commit -m fix",
        "git -c core.hooksPath=/dev/null push",
        "git config core.hooksPath /dev/null",
        "git config --local core.hooksPath .none",
        "SKIP=gitleaks git commit -m fix",
        "HUSKY=0 git commit -m fix",
        "GIT_CONFIG_KEY_0=core.hooksPath git commit -m fix",
        "/usr/bin/git commit --no-verify -m fix",
    ] {
        let blocked = result("bash", command).expect("hook bypass must be blocked");
        assert!(!blocked.success, "{command}");
        assert!(blocked.output.contains("GIT_HOOK_BYPASS_BLOCKED"), "{command}");
    }
}

#[test]
fn allows_ordinary_git_commands() {
    for command in [
        "git commit -m 'add new endpoint'",
        "git commit -m fix -m 'no-verify is not an option here'",
        "git commit -am 'rename fn'",
        "git push origin HEAD:refs/heads/codetether/issue-1",
        "git merge --no-edit origin/main",
        "git config user.name agent",
        "git log -n 5",
        "git diff --name-only",
        "GIT_CONFIG_GLOBAL=/dev/null git status",
        "rg -- '--no-verify' docs",
    ] {
        assert!(result("bash", command).is_none(), "{command}");
    }
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