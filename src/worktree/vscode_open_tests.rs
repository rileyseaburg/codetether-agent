//! Tests for opening worktrees/workspaces in VS Code via a fake binary.

use super::{WorktreeInfo, WorktreeManager};

/// Build a fake "code" binary that exits 0 so we can assert launch success.
fn fake_code_bin(dir: &std::path::Path) -> std::path::PathBuf {
    let bin = dir.join("fake-code.sh");
    std::fs::write(&bin, "#!/bin/sh\nexit 0\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&bin, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    bin
}

fn info_for(dir: &std::path::Path, name: &str) -> WorktreeInfo {
    WorktreeInfo {
        name: name.to_string(),
        path: dir.join(name),
        branch: format!("codetether/{name}"),
        active: true,
    }
}

// Both cases live in one test because they mutate the process-global
// `CODETETHER_VSCODE_BIN` env var; running them in parallel would race.
#[tokio::test]
async fn open_in_vscode_success_and_missing_binary() {
    let dir = tempfile::tempdir().unwrap();
    let mgr = WorktreeManager::for_repo(dir.path());
    let info = info_for(dir.path(), "feature-x");

    // Success: configured binary exists and exits 0.
    let bin = fake_code_bin(dir.path());
    // SAFETY: single-threaded test; sets env for this process only.
    unsafe { std::env::set_var("CODETETHER_VSCODE_BIN", &bin) };
    mgr.open_in_vscode(&info, false).await.unwrap();
    mgr.open_in_vscode(&info, true).await.unwrap();

    // Failure: configured binary does not exist.
    // SAFETY: single-threaded test; sets env for this process only.
    unsafe { std::env::set_var("CODETETHER_VSCODE_BIN", dir.path().join("nope-not-real")) };
    assert!(mgr.open_in_vscode(&info, false).await.is_err());
}
