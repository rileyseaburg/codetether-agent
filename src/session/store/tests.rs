//! Core delta persistence and identity regressions.
mod constraints;
mod consumers;
mod events;
mod eviction;
mod incident;
mod migration;
mod recovery;
mod replacement;
mod scaling;
mod upgrade;
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
fn message(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
#[tokio::test]
async fn append_resume_and_conflict() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.json");
    let mut session = Session::new().await.unwrap();
    session.messages.push(message("one"));
    assert!(super::save(&session, &path).await.unwrap());
    assert!(!super::save(&session, &path).await.unwrap());
    let mut loaded = super::load(&path, 1).await.unwrap().session;
    let mut stale = loaded.clone();
    loaded.messages.push(message("two"));
    super::save(&loaded, &path).await.unwrap();
    stale.messages.push(message("stale"));
    assert!(
        super::save(&stale, &path)
            .await
            .unwrap_err()
            .to_string()
            .contains("REVISION_CONFLICT")
    );
    let tail = super::load(&path, 1).await.unwrap();
    assert_eq!(tail.session.id, session.id);
    assert_eq!(tail.dropped, 1);
    assert_eq!(tail.session.messages.len(), 1);
    let full = super::load(&path, usize::MAX).await.unwrap();
    assert_eq!(full.session.messages.len(), 2);
}
