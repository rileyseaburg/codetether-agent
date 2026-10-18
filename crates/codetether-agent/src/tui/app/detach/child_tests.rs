use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

use super::tail;

#[tokio::test]
async fn detached_child_copies_only_the_configured_tail() {
    let mut parent = Session::new().await.expect("session");
    parent.messages = (0..5).map(message).collect();

    let copied = tail(&parent, 2);

    assert_eq!(text(&copied[0]), "3");
    assert_eq!(text(&copied[1]), "4");
}

fn message(index: usize) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: index.to_string(),
        }],
    }
}

fn text(message: &Message) -> &str {
    match &message.content[0] {
        ContentPart::Text { text } => text,
        _ => panic!("expected text"),
    }
}
