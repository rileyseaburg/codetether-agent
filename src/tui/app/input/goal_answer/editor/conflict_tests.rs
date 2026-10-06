//! Conflicts and validation errors retain the user's unsaved draft.
use super::{fixture, keys};
use crate::session::tasks::runtime::answer_review;

#[tokio::test]
async fn goal_editor_conflict_and_validation_errors_keep_unsaved_text() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let id = slot.view().id().to_string();
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit").await;
    keys::end(&mut app, &mut slot, &runtime).await;
    super::super::paste(&mut app, " my draft");
    crate::tui::app::commands::goal::handle(&mut app, &id, "edit External revision").await;
    keys::save(&mut app, &mut slot, &runtime).await;
    assert!(app.state.goal_editor.is_some());
    assert!(app.state.status.contains("Goal changed while editing"));
    assert_eq!(
        app.state.editor.as_ref().unwrap().text(),
        "Finish the goal my draft"
    );
    assert_eq!(
        answer_review::read(&id).unwrap().goal.unwrap().objective,
        "External revision"
    );
    keys::escape(&mut app, &mut slot, &runtime).await;
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit").await;
    app.state.editor = Some(crate::tui::ui::editor::FileBuffer::draft(
        "Session goal (draft)",
        " ",
    ));
    keys::save(&mut app, &mut slot, &runtime).await;
    assert!(app.state.goal_editor.is_some());
    assert!(app.state.status.contains("objective must contain"));
    assert_eq!(
        answer_review::read(&id).unwrap().goal.unwrap().objective,
        "External revision"
    );
    keys::escape(&mut app, &mut slot, &runtime).await;
    runtime.shutdown().await;
}
