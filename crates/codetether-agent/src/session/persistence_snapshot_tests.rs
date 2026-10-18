use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn borrowed_snapshot_round_trips_without_mutating_sidecars() {
    let mut session = Session::new().await.expect("session");
    session.messages.push(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "retained".into(),
        }],
    });
    assert!(session.pages.is_empty());

    let bytes = super::serialize(&session).expect("serialize");
    let decoded: Session = serde_json::from_slice(&bytes).expect("deserialize");

    let ContentPart::Text { text } = &decoded.messages[0].content[0] else {
        panic!("expected text");
    };
    assert_eq!(text, "retained");
    assert_eq!(decoded.pages.len(), 1);
    assert!(session.pages.is_empty());
}
