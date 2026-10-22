//! Registered pin logic operates on evicted native sources without widening history.
use super::{Session, WINDOW, load, message, save};
use codetether_context_pin::{PinSource, apply_to_session};
use serde_json::json;
mod partial;
mod setup;

#[tokio::test]
async fn historical_pin_and_supersede_use_indexed_sources() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("historical.json");
    save(&setup::session().await, &path).await.unwrap();
    let mut tail = load(&path, WINDOW).await.unwrap().session;
    for action in ["unpin", "pin", "supersede"] {
        let result = apply_to_session(
            &mut tail,
            json!({"action":action,"turn_index":0,"replacement_index":1}),
        )
        .await
        .unwrap();
        assert!(result.success, "{}", result.output);
        assert_eq!(tail.messages.len(), WINDOW);
        tail = load(&path, WINDOW).await.unwrap().session;
        if action == "unpin" {
            assert!(tail.constraint_source(0).is_none());
        }
        if action == "pin" {
            assert!(tail.constraint_source(0).is_some());
        }
    }
    assert_eq!(tail.constraint_source(0).unwrap().superseded_by, Some(1));
    assert_eq!(tail.constraint_source(1).unwrap().text, "Use blue.");
}
