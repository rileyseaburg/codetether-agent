use crate::approval::{ApprovalStatus, LiveApprovalDecision, LiveApprovalRequest};
use crate::tui::app::state::approval_queue;

struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        approval_queue::reset();
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[tokio::test]
async fn edited_patch_replaces_waiting_tool_arguments() {
    let _lock = crate::approval::test_env::lock_env();
    let dir = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", dir.path()) };
    let _guard = Guard;
    let store = crate::approval::ApprovalStore::open_default().unwrap();
    let patch = "--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1 +1 @@\n-old\n+edited\n";
    let resource = crate::tool::patch::approval_resource_for_root(dir.path(), patch);
    let original = store
        .create_request("apply_patch", "write", &resource, "patch write")
        .unwrap();
    let live = LiveApprovalRequest::new(
        original.id.clone(),
        "call".into(),
        "apply_patch".into(),
        "write".into(),
        resource,
        "patch write".into(),
    );
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let waiter = tokio::spawn(async move { crate::approval::live::request(&tx, live).await });
    rx.recv().await.unwrap();

    let status = super::finish::apply(&original.id, patch).unwrap();
    let decision = waiter.await.unwrap();

    let LiveApprovalDecision::Revised {
        arguments,
        approval_id,
    } = decision
    else {
        panic!("expected revised decision")
    };
    assert_eq!(arguments["patch"], patch);
    assert!(status.contains(&approval_id));
    assert_eq!(
        store.decision(&original.id).unwrap().unwrap().status,
        ApprovalStatus::Denied
    );
}
