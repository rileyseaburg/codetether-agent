//! Evicting the working set cannot fork identity or discard durable history.
use super::super::{WINDOW, load, save};
use super::message;
use crate::session::Session;
#[tokio::test]
async fn bounded_working_set_keeps_absolute_sequence_and_prefix() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("window.json");
    let mut session = Session::new().await.unwrap();
    session
        .messages
        .extend((0..WINDOW * 2).map(|_| message("prefix")));
    save(&session, &path).await.unwrap();
    let mut tail = load(&path, WINDOW).await.unwrap().session;
    let id = tail.id.clone();
    tail.add_message(message("new"));
    assert_eq!(tail.messages.len(), WINDOW);
    assert_eq!(tail.message_offset(), WINDOW + 1);
    save(&tail, &path).await.unwrap();
    let full = load(&path, usize::MAX).await.unwrap().session;
    assert_eq!(full.messages.len(), WINDOW * 2 + 1);
    assert_eq!(full.id, id);
    tail.messages.truncate(0);
    save(&tail, &path).await.unwrap();
    assert_eq!(
        load(&path, usize::MAX)
            .await
            .unwrap()
            .session
            .messages
            .len(),
        WINDOW + 1
    );
}
