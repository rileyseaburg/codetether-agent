//! No-op saves add no events, and file switching cannot leak a goal draft to disk.
use super::{fixture, keys};
use crate::session::tasks::TaskLog;

#[tokio::test]
async fn goal_editor_no_change_save_is_noop_and_finder_stays_closed() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let log = TaskLog::for_session(slot.view().id()).unwrap();
    let before = log.read_all().await.unwrap().len();
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit").await;
    keys::press(
        &mut app,
        &mut slot,
        &runtime,
        keys::KeyCode::Char('p'),
        keys::KeyModifiers::CONTROL,
    )
    .await;
    assert!(app.state.goal_editor.is_some());
    assert!(app.state.status.contains("Editing a goal, not a file"));
    keys::save(&mut app, &mut slot, &runtime).await;
    assert!(app.state.goal_editor.is_none());
    assert_eq!(app.state.status, "Goal unchanged");
    assert_eq!(log.read_all().await.unwrap().len(), before);
    runtime.shutdown().await;
}
