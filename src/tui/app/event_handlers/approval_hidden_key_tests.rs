//! Tests that hidden approval overlays do not capture navigation.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::approval::{LiveApprovalRequest, test_env::lock_env};
use crate::tui::app::state::{App, approval_queue};

struct QueueGuard;
impl Drop for QueueGuard {
    fn drop(&mut self) {
        approval_queue::reset();
    }
}

fn app_with_preview() -> App {
    approval_queue::push(
        LiveApprovalRequest::new(
            "approval".into(),
            "call".into(),
            "apply_patch".into(),
            "write".into(),
            "file".into(),
            "patch".into(),
        )
        .with_preview("line\n".repeat(30)),
    );
    App::default()
}

#[test]
fn feedback_input_keeps_navigation_out_of_hidden_preview() {
    let _lock = lock_env();
    approval_queue::reset();
    let _guard = QueueGuard;
    let mut app = app_with_preview();
    app.state.input = "revise this".into();
    let key = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);

    assert!(!super::scroll(&mut app, key));
    assert_eq!(app.state.approval_preview_scroll, 0);
}
