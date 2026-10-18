//! Mocked local rendering regressions for long and narrow approval requests.

use super::test_support::{QueueGuard, draw, queue};
use crate::approval::test_env::lock_env;
use crate::tui::app::state::{App, approval_queue};

#[test]
fn approval_second_command_is_reachable_and_end_scrolls_back_up() {
    let _lock = lock_env();
    approval_queue::reset();
    let _guard = QueueGuard;
    queue(&format!(
        "FIRST_COMMAND\n{}\nSECOND_COMMAND_VISIBLE",
        "long command ".repeat(300)
    ));
    let mut app = App::default();
    assert!(draw(&mut app, 80, 24, "start").contains("FIRST_COMMAND"));
    app.state.approval_preview_scroll = u16::MAX;
    let tail = draw(&mut app, 80, 24, "end");
    assert!(tail.contains("SECOND_COMMAND_VISIBLE"));
    assert!(tail.contains("Ctrl+A approve"));
    let limit = app.state.approval_preview_scroll;
    assert!(limit > 0 && limit < u16::MAX);
    app.state.approval_preview_scroll -= 1;
    draw(&mut app, 80, 24, "end-minus-one");
    assert_eq!(app.state.approval_preview_scroll, limit - 1);
    draw(&mut app, 120, 60, "resized");
    assert!(app.state.approval_preview_scroll < limit);
}

#[test]
fn approval_shortcuts_wrap_and_tiny_terminals_do_not_panic() {
    let _lock = lock_env();
    approval_queue::reset();
    let _guard = QueueGuard;
    queue("first\nsecond");
    let mut app = App::default();
    let text = draw(&mut app, 40, 24, "narrow");
    for hint in ["Ctrl+A", "Ctrl+D", "Tab/Shift+Tab", "Ctrl+E", "Ctrl+Y"] {
        assert!(text.contains(hint), "missing shortcut {hint}");
    }
    for (width, height) in [(1, 1), (8, 4), (36, 10)] {
        draw(&mut app, width, height, "tiny");
    }
}
