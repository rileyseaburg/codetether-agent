use std::sync::Arc;

use crate::{provider::ProviderRegistry, session::Session};

use super::{PromptRequest, SessionNotice, SessionSlot};

/// Verifies that a session moved out of a [`SessionSlot`] can be restored
/// without losing its identity.
///
/// This test exercises the prompt-submission handoff path: the slot temporarily
/// relinquishes ownership of the active session, exposes only its read-only view
/// while empty, and then accepts the moved session back after runtime work
/// finishes or is cancelled.
#[tokio::test]
async fn slot_restores_moved_session() {
    let session = Session::new().await.expect("session");
    let id = session.id.clone();
    let mut slot = SessionSlot::new(session);
    let moved = slot.take_for_prompt().expect("session should move");
    assert!(slot.borrow().is_none());
    assert_eq!(slot.view().id(), id);
    slot.restore(moved);
    assert_eq!(slot.borrow().map(|s| s.id.as_str()), Some(id.as_str()));
}

/// Verifies that the cached session view is refreshed when a moved session is
/// restored to its slot.
///
/// The TUI renders from [`SessionSlot::view`] while prompt execution owns the
/// mutable session. This test ensures metadata changes made during that moved
/// ownership period become visible to renderers after restoration.
#[tokio::test]
async fn view_updates_after_restore() {
    let mut session = Session::new().await.expect("session");
    session.metadata.model = Some("openai/gpt-4o".into());
    let mut slot = SessionSlot::new(session);
    let mut moved = slot.take_for_prompt().expect("session should move");
    moved.metadata.model = Some("zai/glm-5".into());
    slot.restore(moved);
    assert_eq!(slot.view().model.as_deref(), Some("zai/glm-5"));
}

/// Verifies that runtime prompt failures return ownership of the submitted
/// session to the caller through a failure notice.
///
/// The test uses an empty [`ProviderRegistry`] so prompt execution cannot find a
/// provider. That controlled failure should emit [`SessionNotice::Started`]
/// followed by [`SessionNotice::Failed`] carrying the original session, allowing
/// the TUI to restore state instead of dropping the conversation.
#[tokio::test]
async fn runtime_failure_returns_session() {
    let session = Session::new().await.expect("session");
    let id = session.id.clone();
    let (event_tx, _event_rx) = tokio::sync::mpsc::channel(4);
    let (notice_tx, mut notice_rx) = tokio::sync::mpsc::channel(4);
    let runtime = super::spawn(event_tx, notice_tx);
    let request = PromptRequest::new(
        session,
        "hello".into(),
        Vec::new(),
        Arc::new(ProviderRegistry::new()),
        None,
        None,
    );
    assert!(runtime.submit(request).await.is_ok());
    let started = notice_rx.recv().await;
    assert!(matches!(started, Some(SessionNotice::Started)));
    let Some(SessionNotice::Failed { session, error }) = notice_rx.recv().await else {
        panic!("expected failed notice");
    };
    assert_eq!(session.id, id);
    assert!(error.contains("No providers available"));
    runtime.shutdown().await;
}
