//! Y/N typed into the editor is text, not acceptance or tool approval.
use super::{fixture, keys};
use crate::session::tasks::{AnswerReviewAction, runtime::answer_review};

#[tokio::test]
async fn goal_editor_keyboard_edits_text_without_accepting_a_review() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let id = slot.view().id().to_string();
    answer_review::begin(&id, "Why?").await.unwrap();
    let review = answer_review::read(&id).unwrap().answer_review.unwrap();
    answer_review::record(
        &id,
        &review.goal_id,
        &review.id,
        AnswerReviewAction::Answered,
    )
    .await
    .unwrap();
    app.state.approval_waiting = true;
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit").await;
    keys::end(&mut app, &mut slot, &runtime).await;
    keys::press(
        &mut app,
        &mut slot,
        &runtime,
        keys::KeyCode::Char('Y'),
        keys::KeyModifiers::NONE,
    )
    .await;
    assert_eq!(
        app.state.editor.as_ref().unwrap().text(),
        "Finish the goalY"
    );
    assert!(answer_review::ready(&id));
    keys::save(&mut app, &mut slot, &runtime).await;
    assert!(answer_review::ready(&id));
    assert!(app.state.approval_waiting);
    assert_eq!(
        answer_review::read(&id).unwrap().goal.unwrap().objective,
        "Finish the goalY"
    );
    runtime.shutdown().await;
}
