//! Normal cursor/backspace edits preserve the untouched portion of a live goal.
use super::{fixture, keys};
use crate::session::tasks::runtime::answer_review;

#[tokio::test]
async fn goal_editor_changes_a_word_in_place() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let id = slot.view().id().to_string();
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit").await;
    for _ in 0..6 {
        keys::press(
            &mut app,
            &mut slot,
            &runtime,
            keys::KeyCode::Right,
            keys::KeyModifiers::NONE,
        )
        .await;
    }
    assert!(!app.state.editor.as_ref().unwrap().is_dirty());
    for _ in 0..6 {
        keys::press(
            &mut app,
            &mut slot,
            &runtime,
            keys::KeyCode::Backspace,
            keys::KeyModifiers::NONE,
        )
        .await;
    }
    super::super::paste(&mut app, "Review");
    assert_eq!(app.state.editor.as_ref().unwrap().text(), "Review the goal");
    assert_eq!(
        answer_review::read(&id).unwrap().goal.unwrap().objective,
        "Finish the goal"
    );
    keys::save(&mut app, &mut slot, &runtime).await;
    assert_eq!(
        answer_review::read(&id).unwrap().goal.unwrap().objective,
        "Review the goal"
    );
    runtime.shutdown().await;
}
