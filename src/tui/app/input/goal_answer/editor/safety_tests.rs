//! Preserve unrelated drafts and refuse writes to a different active session.
use super::{fixture, keys};
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::ui::editor::FileBuffer;

#[tokio::test]
async fn goal_editor_does_not_replace_an_existing_file_draft() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    app.state.editor = Some(FileBuffer::proposed("unsaved.rs", "keep my edits"));
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit").await;
    assert!(app.state.goal_editor.is_none());
    assert_eq!(app.state.editor.as_ref().unwrap().text(), "keep my edits");
    assert!(app.state.status.contains("Close the current editor"));
    runtime.shutdown().await;
}

#[tokio::test]
async fn goal_editor_rejects_saving_into_a_different_session() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit").await;
    keys::end(&mut app, &mut slot, &runtime).await;
    super::super::paste(&mut app, " draft");
    let other = crate::session::tasks::answer_review_test_support::session().await;
    let mut other_slot = SessionSlot::new(other);
    keys::save(&mut app, &mut other_slot, &runtime).await;
    assert!(app.state.goal_editor.is_some());
    assert!(app.state.status.contains("Session changed"));
    assert_eq!(
        app.state.editor.as_ref().unwrap().text(),
        "Finish the goal draft"
    );
    keys::escape(&mut app, &mut other_slot, &runtime).await;
    runtime.shutdown().await;
}
