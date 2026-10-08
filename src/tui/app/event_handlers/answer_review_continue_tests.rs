//! Typing continuation reaches chat input instead of being eaten by the overlay.
use super::super::super::handle;
use super::super::support;
use crate::session::tasks::runtime::answer_review;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[tokio::test]
async fn answer_review_continuation_keys_do_not_accept_tools_or_answers() {
    for text in ["continue", "coninue", "resume", "/continue"] {
        let (mut app, slot) = support::ready().await;
        app.state.approval_waiting = true;
        for c in text.chars() {
            let key = KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE);
            assert_eq!(handle(&mut app, &slot, key).await.unwrap(), None);
            app.state.input.push(c);
        }
        let enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        assert_eq!(handle(&mut app, &slot, enter).await.unwrap(), None);
        assert!(answer_review::held(slot.view().id()));
        assert!(app.state.approval_waiting);
    }
}
