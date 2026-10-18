//! Accepting an answer is not accepting a pending tool approval.

use crate::session::tasks::{GoalStatus, runtime::answer_review};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[tokio::test]
async fn answer_review_yes_releases_hold_but_not_tool_approval() {
    let (mut app, mut slot) = super::support::ready().await;
    app.state.approval_waiting = true;
    let ctrl_y = KeyEvent::new(KeyCode::Char('y'), KeyModifiers::CONTROL);
    assert_eq!(
        super::super::handle(&mut app, &mut slot, ctrl_y)
            .await
            .unwrap(),
        Some(false)
    );
    assert!(answer_review::held(slot.view().id()));
    assert!(answer_review::ready(slot.view().id()));
    let y = KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE);
    assert_eq!(
        super::super::handle(&mut app, &mut slot, y).await.unwrap(),
        Some(true)
    );
    assert!(!answer_review::held(slot.view().id()));
    assert!(app.state.approval_waiting);
    let goal = answer_review::read(slot.view().id()).unwrap().goal.unwrap();
    assert_eq!(goal.status, GoalStatus::Active);
}
