//! Verify LSP errors are returned before a live approval event is emitted.

use serde_json::json;
use tokio::sync::mpsc;
use tokio::time::{Duration, timeout};

struct DataDirGuard;

impl Drop for DataDirGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
        unsafe { std::env::remove_var("XDG_CONFIG_HOME") };
    }
}

#[tokio::test]
async fn typescript_error_never_reaches_approval_channel() {
    if which::which("typescript-language-server").is_err() {
        return;
    }
    let _lock = crate::approval::test_env::lock_env();
    let dir = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", dir.path()) };
    unsafe { std::env::set_var("XDG_CONFIG_HOME", dir.path().join("config")) };
    let _env = DataDirGuard;
    let config_path = crate::config::Config::global_config_path().unwrap();
    std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    std::fs::write(config_path, "approval_policy = 'on-request'").unwrap();
    std::fs::write(dir.path().join("tsconfig.json"), "{}").unwrap();
    let path = dir.path().join("broken.ts");
    std::fs::write(&path, "export {};\n").unwrap();
    let args = json!({
        "path": path,
        "content": "export const broken: string = 42;\n",
        "__ct_parent_workspace": dir.path()
    });
    let (tx, mut rx) = mpsc::channel(1);

    let gate = timeout(
        Duration::from_secs(5),
        super::gate::gate(dir.path(), &tx, "call-1", "write", args),
    )
    .await
    .expect("LSP gate waited for user approval");
    let (_, blocked) = gate.into_parts();
    let (output, success, _) = blocked.expect("LSP error did not block the write");

    assert!(!success);
    assert!(output.contains("LSP_PREAPPROVAL_FAILED"));
    assert!(rx.try_recv().is_err(), "approval event was emitted");
}
