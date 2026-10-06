//! Goal commands remain reachable while satisfaction or approval is pending.

use crate::session::tasks::runtime::answer_review;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[tokio::test]
async fn answer_review_allows_goal_commands_without_accepting_review() {
    let (mut app, slot) = super::support::ready().await;
    app.state.approval_waiting = true;
    let slash = KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE);
    assert_eq!(
        super::super::handle(&mut app, &slot, slash).await.unwrap(),
        None
    );
    app.state.input = "/goal override revised objective".into();
    let enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
    assert_eq!(
        super::super::handle(&mut app, &slot, enter).await.unwrap(),
        None
    );
    assert!(answer_review::held(slot.view().id()));
    assert!(answer_review::ready(slot.view().id()));
    assert!(app.state.approval_waiting);
}
