//! Recall projection construction tests.

use crate::provider::{ContentPart, Message, Role};

#[tokio::test]
async fn projection_keeps_absolute_turn_provenance() {
    let mut session = crate::session::Session::new().await.unwrap();
    session.id = "recall-build".into();
    session.messages = (0..6).map(message).collect();
    let indexed = super::super::build::session(&session).unwrap();
    assert_eq!(indexed.message_count, 6);
    assert_eq!(indexed.documents.len(), 2);
    assert_eq!(indexed.documents[1].start, 4);
    assert!(indexed.documents[1].excerpt.contains("[4 User]"));
}

fn message(index: usize) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: format!("decision-{index}"),
        }],
    }
}
