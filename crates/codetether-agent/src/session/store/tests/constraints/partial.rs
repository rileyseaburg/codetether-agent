//! Exact corrections survive native metadata storage and evicted source lookup.
use super::{Session, WINDOW, load, message, save};
use codetether_context_pin::PinSource;
use codetether_provider_core::ContentPart;
mod setup;

#[tokio::test]
async fn partial_correction_survives_eviction_and_repeated_cold_reloads() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("partial.json");
    let session = setup::session().await;
    save(&session, &path).await.unwrap();
    for _ in 0..3 {
        let mut tail = load(&path, WINDOW).await.unwrap().session;
        assert_eq!(tail.corrections().len(), 1);
        assert!(tail.constraint_source(0).unwrap().text.contains("Use red."));
        let mut messages = vec![];
        codetether_context_dispatch::prepend_state_header(&tail, &mut messages);
        let ContentPart::Text { text } = &messages[0].content[0] else {
            panic!()
        };
        assert!(text.contains("Keep evidence."));
        assert!(text.contains("Do not deploy."));
        assert!(text.contains("Use blue."));
        assert!(!text.contains("Use red."));
        tail.add_message(message("continue"));
        save(&tail, &path).await.unwrap();
    }
}
