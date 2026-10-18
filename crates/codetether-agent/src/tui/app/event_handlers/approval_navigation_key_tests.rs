//! Browsing pending approvals must neither decide nor lose requests.

use crate::approval::{LiveApprovalRequest, test_env::lock_env};
use crate::tui::app::state::{App, approval_queue};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        approval_queue::reset();
    }
}

#[test]
fn approval_tab_and_backtab_browse_requests_without_resolving() {
    let _lock = lock_env();
    approval_queue::reset();
    let _guard = Guard;
    for id in ["one", "two"] {
        approval_queue::push(LiveApprovalRequest::new(
            id.into(),
            id.into(),
            "bash".into(),
            "execute".into(),
            id.into(),
            "reason".into(),
        ));
    }
    let mut app = App::default();
    app.state.approval_preview_scroll = 20;
    for (key, expected) in [(KeyCode::Tab, "two"), (KeyCode::BackTab, "one")] {
        assert!(super::scroll(
            &mut app,
            KeyEvent::new(key, KeyModifiers::NONE)
        ));
        assert_eq!(approval_queue::active_id().as_deref(), Some(expected));
        assert_eq!(approval_queue::len(), 2);
        assert_eq!(app.state.approval_preview_scroll, 0);
    }
    assert!(super::scroll(
        &mut app,
        KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT)
    ));
    assert_eq!(approval_queue::active_id().as_deref(), Some("two"));
}
