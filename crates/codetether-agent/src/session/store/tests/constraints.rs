//! Pinned scope survives bounded resume and later working-set eviction.
use super::super::{WINDOW, constraints, load, save};
use super::message;
use crate::session::{Session, pages::PageKind};
#[tokio::test]
async fn old_pinned_constraints_remain_in_the_compact_state_header() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("pins.json");
    let mut session = Session::new().await.unwrap();
    session.messages.push(message("never delete evidence"));
    session
        .messages
        .extend((0..WINDOW).map(|_| message("later")));
    session.pages = vec![PageKind::Conversation; WINDOW + 1].into();
    session.pages[0] = PageKind::Constraint;
    save(&session, &path).await.unwrap();
    let mut tail = load(&path, WINDOW).await.unwrap().session;
    assert_eq!(
        constraints::prefix(&tail),
        vec!["- turn 0: never delete evidence"]
    );
    tail.add_message(message("continue"));
    save(&tail, &path).await.unwrap();
    let tail = load(&path, WINDOW).await.unwrap().session;
    assert_eq!(
        constraints::prefix(&tail),
        vec!["- turn 0: never delete evidence"]
    );
}
