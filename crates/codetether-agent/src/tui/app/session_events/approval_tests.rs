//! Tests for transient approval request presentation.

use crate::approval::{LiveApprovalRequest, test_env::lock_env};
use crate::tui::app::state::{App, approval_queue};

struct QueueGuard;
impl Drop for QueueGuard {
    fn drop(&mut self) {
        approval_queue::reset();
    }
}

#[test]
fn request_uses_overlay_and_status_without_polluting_transcript() {
    let _lock = lock_env();
    approval_queue::reset();
    let _guard = QueueGuard;
    let mut app = App::default();
    let message_count = app.state.messages.len();
    let request = LiveApprovalRequest::new(
        "approval".into(),
        "call".into(),
        "bash".into(),
        "execute".into(),
        "bash:echo".into(),
        "test".into(),
    );

    super::request(&mut app, request);

    assert_eq!(app.state.messages.len(), message_count);
    assert!(app.state.status.starts_with("Approval pending (1)"));
    assert!(app.state.approval_waiting);
    assert_eq!(approval_queue::len(), 1);
}
