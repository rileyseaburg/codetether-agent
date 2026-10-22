//! Fork a persisted tail containing absolute correction links.
use super::{Session, WINDOW, load, message, save};
use codetether_provider_core::ContentPart;
mod setup;

#[tokio::test]
async fn fork_rebases_native_tail_corrections_and_preserves_pins() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("fork-parent.json");
    let parent = setup::session().await;
    save(&parent, &path).await.unwrap();
    let tail = load(&path, WINDOW).await.unwrap().session;
    let cut = tail.messages.len() - 1;
    let child = codetether_session_fork::create(&tail, cut, "Continue")
        .await
        .unwrap();
    let correction = &child.metadata.constraint_corrections[0];
    assert_eq!(correction.source_turn, cut - 2);
    assert_eq!(correction.replacement_turn, cut - 1);
    assert_eq!(tail.metadata.constraint_corrections[0].source_turn, WINDOW);
    let mut rendered = vec![];
    codetether_context_dispatch::prepend_state_header(&child, &mut rendered);
    let ContentPart::Text { text } = &rendered[0].content[0] else {
        panic!()
    };
    assert!(text.contains("Keep evidence."));
    assert!(text.contains("Use blue."));
    assert!(!text.contains("Use red."));
}
