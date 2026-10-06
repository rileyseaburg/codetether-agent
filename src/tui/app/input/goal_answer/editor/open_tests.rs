//! A busy goal opens prefilled without changing or cancelling the live turn.
use super::{fixture, keys};
use crate::session::tasks::{TaskLog, runtime::answer_review};
use crate::tui::models::ViewMode;

#[tokio::test]
async fn goal_editor_prefills_busy_goal_and_escape_discards_the_draft() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let id = slot.view().id().to_string();
    let log = TaskLog::for_session(&id).unwrap();
    let before = log.read_all().await.unwrap().len();
    let session = slot.take_for_prompt().unwrap();
    app.state.processing = true;
    app.state.main_inflight_prompt = Some("Original work".into());
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit").await;
    assert_eq!(app.state.view_mode, ViewMode::Editor);
    assert_eq!(app.state.editor.as_ref().unwrap().text(), "Finish the goal");
    assert!(!app.state.editor.as_ref().unwrap().is_dirty());
    assert!(app.state.processing);
    assert_eq!(
        app.state.main_inflight_prompt.as_deref(),
        Some("Original work")
    );
    keys::end(&mut app, &mut slot, &runtime).await;
    super::super::paste(&mut app, " changed");
    assert_eq!(
        answer_review::read(&id).unwrap().goal.unwrap().objective,
        "Finish the goal"
    );
    keys::escape(&mut app, &mut slot, &runtime).await;
    assert!(app.state.goal_editor.is_none());
    assert_eq!(log.read_all().await.unwrap().len(), before);
    slot.restore(session);
    runtime.shutdown().await;
}
