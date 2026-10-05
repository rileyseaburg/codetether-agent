//! Keyboard and mouse scrolling uses rendered bounds, including fallback text.

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
fn approval_end_then_up_moves_immediately_and_wheel_stays_in_modal() {
    let _lock = lock_env();
    approval_queue::reset();
    let _guard = Guard;
    approval_queue::push(LiveApprovalRequest::new(
        "one".into(),
        "one".into(),
        "bash".into(),
        "execute".into(),
        "fallback resource".into(),
        "reason".into(),
    ));
    approval_queue::set_scroll_limit("one", 15);
    let mut app = App::default();
    for (key, expected) in [(KeyCode::End, 15), (KeyCode::Up, 14), (KeyCode::Down, 15)] {
        assert!(super::scroll(
            &mut app,
            KeyEvent::new(key, KeyModifiers::NONE)
        ));
        assert_eq!(app.state.approval_preview_scroll, expected);
    }
    assert!(super::wheel(&mut app, KeyCode::Up, 3));
    assert_eq!(app.state.approval_preview_scroll, 12);
    assert_eq!(app.state.chat_scroll, 0);
    app.state.show_help = true;
    assert!(!super::wheel(&mut app, KeyCode::Down, 3));
    app.state.show_help = false;
    app.state.input = "/feedback correction".into();
    assert!(!super::scroll(
        &mut app,
        KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE)
    ));
}
