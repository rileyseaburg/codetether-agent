use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::approval::{LiveApprovalRequest, test_env::lock_env};
use crate::tui::app::state::{App, approval_queue};

struct QueueGuard;

impl Drop for QueueGuard {
    fn drop(&mut self) {
        approval_queue::reset();
    }
}

#[test]
fn page_down_scrolls_full_approval_preview() {
    let _lock = lock_env();
    approval_queue::reset();
    let _guard = QueueGuard;
    approval_queue::push(
        LiveApprovalRequest::new(
            "approval-1".into(),
            "call-1".into(),
            "apply_patch".into(),
            "write".into(),
            "file".into(),
            "patch".into(),
        )
        .with_preview("line\n".repeat(30)),
    );
    let mut app = App::default();

    assert!(super::scroll(
        &mut app,
        KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE),
    ));

    assert_eq!(app.state.approval_preview_scroll, 10);
}
