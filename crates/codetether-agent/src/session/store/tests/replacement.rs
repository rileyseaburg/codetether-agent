//! Replacing a clean vector from another loaded session must not evade dirty tracking.
use super::super::{load, save};
use super::message;
use crate::session::Session;
#[tokio::test]
async fn clean_vector_replacement_is_persisted() {
    let root = tempfile::tempdir().unwrap();
    let first = root.path().join("first.json");
    let second = root.path().join("second.json");
    let mut a = Session::new().await.unwrap();
    let mut b = Session::new().await.unwrap();
    a.messages.push(message("first"));
    b.messages.push(message("second"));
    save(&a, &first).await.unwrap();
    save(&b, &second).await.unwrap();
    a.messages = b.messages.clone();
    assert!(save(&a, &first).await.unwrap());
    let loaded = load(&first, 1).await.unwrap().session;
    assert_eq!(
        serde_json::to_value(&loaded.messages).unwrap(),
        serde_json::to_value(&b.messages).unwrap()
    );
}
#[tokio::test]
async fn changed_pages_are_not_evicted_before_persistence() {
    let mut session = Session::new().await.unwrap();
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("pages.json");
    session.messages.extend((0..1100).map(|_| message("old")));
    save(&session, &path).await.unwrap();
    let mut session = load(&path, usize::MAX).await.unwrap().session;
    session.pages[0] = crate::session::pages::PageKind::Constraint;
    session.add_message(message("new"));
    assert_eq!(session.message_offset(), 0);
    save(&session, &path).await.unwrap();
    assert_eq!(
        load(&path, usize::MAX).await.unwrap().session.pages[0],
        crate::session::pages::PageKind::Constraint
    );
}
