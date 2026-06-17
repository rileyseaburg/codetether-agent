//! Tests for best-effort automatic VS Code opening on worktree creation.

use super::{WorktreeInfo, WorktreeManager};

fn info_for(dir: &std::path::Path, name: &str) -> WorktreeInfo {
    WorktreeInfo {
        name: name.to_string(),
        path: dir.join(name),
        branch: format!("codetether/{name}"),
        active: true,
    }
}

// Cases share the process-global env vars, so they run in one serial test.
#[tokio::test]
async fn auto_open_is_best_effort_and_respects_disable_flag() {
    let dir = tempfile::tempdir().unwrap();
    let mgr = WorktreeManager::for_repo(dir.path());
    let info = info_for(dir.path(), "feature-auto");

    // Disabled: no-op regardless of binary. Must not panic or error.
    // SAFETY: single-threaded test; sets env for this process only.
    unsafe { std::env::set_var("CODETETHER_WORKTREE_AUTO_VSCODE", "0") };
    mgr.auto_open_in_vscode(&info).await;

    // Enabled with a missing binary: still best-effort (no panic, no error).
    // SAFETY: single-threaded test; sets env for this process only.
    unsafe {
        std::env::set_var("CODETETHER_WORKTREE_AUTO_VSCODE", "1");
        std::env::set_var("CODETETHER_VSCODE_BIN", dir.path().join("nope-not-real"));
    }
    mgr.auto_open_in_vscode(&info).await;
}

#[tokio::test]
async fn without_vscode_auto_open_disables_prompt() {
    let dir = tempfile::tempdir().unwrap();
    let mgr = WorktreeManager::for_repo(dir.path()).without_vscode_auto_open();
    let info = info_for(dir.path(), "feature-tui");

    // Even with auto-open enabled via env, the manager flag wins and the
    // interactive prompt is skipped (it would otherwise block a non-TTY read).
    // SAFETY: single-threaded test; sets env for this process only.
    unsafe { std::env::set_var("CODETETHER_WORKTREE_AUTO_VSCODE", "1") };
    mgr.auto_open_in_vscode(&info).await;
}

#[tokio::test]
async fn tui_active_skips_prompt() {
    let dir = tempfile::tempdir().unwrap();
    let info = info_for(dir.path(), "feature-tui-active");

    // With the TUI owning the terminal, the raw stdout prompt must never fire.
    // confirm_open checks the TUI flag first, before any env or TTY logic.
    super::set_tui_active(true);
    assert!(!super::vscode_prompt::confirm_open(&info));
    super::set_tui_active(false);
}
