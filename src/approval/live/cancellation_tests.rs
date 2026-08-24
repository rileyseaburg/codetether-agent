use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::approval::{ApprovalStatus, ApprovalStore, LiveApprovalRequest};

#[tokio::test]
async fn cancelled_waiter_denies_durable_request() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), crate::config::AccessMode::Ask);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:cancel", "cancel")
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
    waiter.abort();
    tokio::task::yield_now().await;
    assert_eq!(
        store.decision(&request.id).unwrap().unwrap().status,
        ApprovalStatus::Denied
    );
}
