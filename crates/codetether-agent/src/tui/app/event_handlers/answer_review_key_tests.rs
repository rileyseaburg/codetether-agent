//! Exercise the real satisfaction-key handler, independently of tool approval.

use crate::session::tasks::runtime::answer_review;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
#[path = "answer_review_test_support.rs"]
mod support;

#[tokio::test]
async fn answer_review_enter_defaults_to_no_and_keeps_hold() {
    let (mut app, mut slot) = support::ready().await;
    assert!(!app.state.answer_review_yes);
    let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
    assert_eq!(
        super::handle(&mut app, &mut slot, key).await.unwrap(),
        Some(false)
    );
    assert!(answer_review::held(slot.view().id()));
    assert!(!answer_review::ready(slot.view().id()));
}

#[path = "answer_review_accept_tests.rs"]
mod accept;
#[path = "answer_review_command_tests.rs"]
mod commands;
