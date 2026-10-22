//! Metadata and an evicted replacement pin commit together through the tool entry.
use super::*;
use codetether_provider_core::ContentPart;
#[tokio::test]
async fn historical_partial_correction_commits_metadata_and_pin_together() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("partial-history.json");
    save(&setup::session().await, &path).await.unwrap();
    let mut tail = load(&path, WINDOW).await.unwrap().session;
    let hash = tail.constraint_source(0).unwrap().text_hash;
    let result = apply_to_session(&mut tail, json!({"action":"correct","turn_index":0,"replacement_index":1,"source_text":"Use red.","source_hash":hash})).await.unwrap();
    assert!(result.success, "{}", result.output);
    let reloaded = load(&path, WINDOW).await.unwrap().session;
    assert_eq!(reloaded.messages.len(), WINDOW);
    assert_eq!(reloaded.metadata.constraint_corrections.len(), 1);
    let mut rendered = vec![];
    codetether_context_dispatch::prepend_state_header(&reloaded, &mut rendered);
    let ContentPart::Text { text } = &rendered[0].content[0] else {
        panic!()
    };
    assert!(text.contains("Keep evidence."));
    assert!(text.contains("Never deploy."));
    assert!(text.contains("Use blue."));
    assert!(!text.contains("Use red."));
}
