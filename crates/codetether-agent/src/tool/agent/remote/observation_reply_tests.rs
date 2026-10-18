//! The transcript records what the peer said, not the model-facing envelope.

use crate::a2a::types::{Message, MessageRole, Part, SendMessageResponse};
use crate::tool::agent::message::remote::reply::PeerReply;
use crate::tool::agent::message::remote::result;
use crate::tui::app::message_text::extract_message_text;

fn response(text: &str) -> SendMessageResponse {
    SendMessageResponse::Message(Message {
        message_id: "m1".into(),
        role: MessageRole::Agent,
        parts: vec![Part::Text { text: text.into() }],
        context_id: None,
        task_id: None,
        metadata: Default::default(),
        extensions: Vec::new(),
    })
}

#[test]
fn transcript_holds_the_plain_reply_while_the_model_gets_the_envelope() {
    let name = format!("peer-{}", uuid::Uuid::new_v4());
    let reply = PeerReply::from_response(&response("Static/local: the hook works."));
    let rendered = result::render(&name, &reply);
    assert!(rendered.output.contains("\"transport\": \"a2a-mdns\""));

    let turn = super::begin(&name, Some("owner"), "help");
    turn.settle(&reply);
    let transcript = super::transcript(&name, "owner").unwrap();
    let last = extract_message_text(&transcript.last().unwrap().content);
    assert_eq!(last, "Static/local: the hook works.");
    assert!(
        !last.contains('{'),
        "envelope leaked into transcript: {last}"
    );
}

#[test]
fn empty_reply_is_named_and_failure_state_is_kept() {
    let reply = PeerReply::from_response(&response("   "));
    assert_eq!(reply.text, "Peer completed without a text response");
    assert!(!reply.failed);
    let failure = PeerReply::transport_failure("did not finish within 120s");
    assert!(failure.failed);
    assert!(failure.text.starts_with("Remote call failed: "));
}
