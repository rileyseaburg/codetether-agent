//! Tests for releasing paused requests after entering full access.

use super::super::test_support::EnvGuard;
use crate::approval::{ApprovalStatus, LiveApprovalRequest, test_env::lock_env};
use crate::tui::app::state::{App, approval_queue};

#[test]
fn full_access_releases_every_queued_request() {
    let _lock = lock_env();
    let data = tempfile::tempdir().unwrap();
    let _env = EnvGuard::new(data.path());
    let store = crate::approval::ApprovalStore::open_default().unwrap();
    let first = store
        .create_request("bash", "execute", "bash:one", "test")
        .unwrap();
    let second = store
        .create_request("bash", "execute", "bash:two", "test")
        .unwrap();
    for request in [&first, &second] {
        approval_queue::push(LiveApprovalRequest::new(
            request.id.clone(),
            format!("call-{}", request.id),
            "bash".into(),
            "execute".into(),
            request.resource.clone(),
            "test".into(),
        ));
    }
    let mut app = App::default();

    assert_eq!(super::all(&mut app).unwrap(), 2);
    assert!(approval_queue::active().is_none());
    assert!(!app.state.approval_waiting);
    assert_eq!(
        store.decision(&first.id).unwrap().unwrap().status,
        ApprovalStatus::Approved
    );
    assert_eq!(
        store.decision(&second.id).unwrap().unwrap().status,
        ApprovalStatus::Approved
    );
}
