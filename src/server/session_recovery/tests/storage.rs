//! Process-isolated, real SQLite-backed session fixtures; no global env mutation.
use crate::{
    provider::{ContentPart, Message, Role},
    session::Session,
};
use std::process::Command;

/// Run a crate-qualified test name as an isolated libtest selection.
/// # Panics
/// Panics if setup fails, the child fails, or exactly one test does not succeed.
pub(super) fn isolated(test: &str) -> bool {
    if std::env::var_os("CODETETHER_RECOVERY_TEST_CHILD").is_some() {
        return false;
    }
    // `module_path!()` includes the crate; libtest's names do not.
    let (_, test) = test.split_once("::").expect("crate-qualified test name");
    let root = tempfile::tempdir().unwrap();
    let workspace = root.path().join("workspace");
    std::fs::create_dir(&workspace).unwrap();
    let output = Command::new(std::env::current_exe().unwrap())
        .args([test, "--exact", "--nocapture", "--test-threads=1"])
        .current_dir(&workspace)
        .env("CODETETHER_RECOVERY_TEST_CHILD", "1")
        .env("CODETETHER_DATA_DIR", root.path().join("data"))
        .env_remove("CODETETHER_HISTORY_SINK_URL")
        .env_remove("CODETETHER_HISTORY_SINK_TOKEN")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("test result: ok. 1 passed;"),
        "Expected one test: {test}\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    true
}

pub(super) async fn durable() -> Session {
    let mut session = Session::new().await.unwrap();
    session.messages.push(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "Retain this durable history".into(),
        }],
    });
    session.metadata.model = Some("original-model".into());
    session.save().await.unwrap();
    session
}
