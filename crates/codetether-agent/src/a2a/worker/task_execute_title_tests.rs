//! Tests for worker session title preseeding.

use super::task_execute_title::preseed_task_title;

#[tokio::test]
async fn preseeds_missing_worker_session_title() {
    let mut session = crate::session::Session::new().await.expect("session");

    preseed_task_title(&mut session, "Forgejo PR review");

    assert_eq!(session.title.as_deref(), Some("Forgejo PR review"));
}

#[tokio::test]
async fn preserves_resumed_session_title() {
    let mut session = crate::session::Session::new().await.expect("session");
    session.set_title("Existing session");

    preseed_task_title(&mut session, "Replacement task title");

    assert_eq!(session.title.as_deref(), Some("Existing session"));
}
