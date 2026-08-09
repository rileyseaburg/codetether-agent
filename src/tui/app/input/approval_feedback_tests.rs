use crate::approval::{ApprovalStatus, LiveApprovalRequest};
use crate::tui::app::state::{App, approval_queue};

struct EnvGuard;
impl Drop for EnvGuard {
    fn drop(&mut self) {}
}

#[test]
fn direct_text_denies_with_revision_reason() {
    let _lock = crate::approval::test_env::lock_env();
    approval_queue::reset();
    let dir = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", dir.path()) };
    let _env = EnvGuard;
    let store = crate::approval::ApprovalStore::open_default().unwrap();
    let request = store
        .create_request("apply_patch", "write", "src/lib.rs", "review")
        .unwrap();
    approval_queue::push(LiveApprovalRequest::new(
        request.id.clone(),
        "call".into(),
        "apply_patch".into(),
        "write".into(),
        "src/lib.rs".into(),
        "review".into(),
    ));
    let mut app = App::default();

    app.state.input = "rename this before applying".into();
    assert!(super::submit(&mut app));

    let decision = store.decision(&request.id).unwrap().unwrap();
    assert_eq!(decision.status, ApprovalStatus::Denied);
    assert_eq!(decision.reason, "rename this before applying");
    approval_queue::reset();
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}
