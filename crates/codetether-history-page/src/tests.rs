//! Page ordering and boundary selection regression.
use crate::select::Decision;
use crate::{anchor, select, types::Request};
use codetether_provider_core::{ContentPart, Message, Role};

#[test]
fn selects_preceding_messages_in_chronological_order() {
    let messages: Vec<_> = (0..300).map(message).collect();
    let request = Request {
        generation: 1,
        source_id: "unused".into(),
        boundary: anchor::fingerprints(&messages[275..]),
        depth: 25,
        before: None,
    };
    let Decision::Ready(page) = select::page(&messages, &request, 400) else {
        panic!()
    };
    assert_eq!(text(&page.messages[0]), "25");
    assert_eq!(text(&page.messages[249]), "274");
}

fn message(value: usize) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: value.to_string(),
        }],
    }
}

fn text(message: &Message) -> &str {
    match &message.content[0] {
        ContentPart::Text { text } => text,
        _ => unreachable!(),
    }
}
