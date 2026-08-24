//! Durable-store failures terminate live waiters fail-closed.

use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::approval::{ApprovalStore, LiveApprovalDecision, LiveApprovalRequest};
use std::io::Write;

#[tokio::test]
async fn corrupt_store_denies_waiter_instead_of_hanging() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), crate::config::AccessMode::Ask);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:corrupt", "corrupt")
        .expect("request");
    let live = LiveApprovalRequest::new(
        request.id.clone(),
        "call".into(),
        "bash".into(),
        "execute".into(),
        "resource".into(),
        "reason".into(),
    );
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let waiter = tokio::spawn(async move { super::request(&tx, live).await });
    rx.recv().await.expect("approval event");
    let mut log = std::fs::OpenOptions::new()
        .append(true)
        .open(data.path().join("approvals/approvals.jsonl"))
        .expect("log");
    writeln!(log, "{{corrupt").expect("corrupt");

    let decision = tokio::time::timeout(std::time::Duration::from_secs(1), waiter)
        .await
        .expect("waiter timeout")
        .expect("waiter task");
    assert!(matches!(
        decision,
        LiveApprovalDecision::Denied { reason: Some(_) }
    ));
}
