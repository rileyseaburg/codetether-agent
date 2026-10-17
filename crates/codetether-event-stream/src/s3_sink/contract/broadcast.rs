//! broadcast contract support.
use crate::{ChatEvent, Event, EventStream};
#[tokio::test]
async fn delivery_and_no_subscriber_error() {
    let stream = EventStream::new();
    assert!(
        stream
            .send(Event::AgentMessage {
                content: "lost".into()
            })
            .is_err()
    );
    let mut first = stream.subscribe();
    let mut second = stream.subscribe();
    stream
        .send(Event::Chat(ChatEvent::UserMessage {
            content: "hello".into(),
            timestamp: 7,
        }))
        .unwrap();
    for message in [first.recv().await.unwrap(), second.recv().await.unwrap()] {
        assert!(matches!(
            message,
            Event::Chat(ChatEvent::UserMessage { timestamp: 7, .. })
        ));
    }
}
#[tokio::test]
async fn capacity_and_lag_are_preserved() {
    let stream = EventStream::default();
    let mut receiver = stream.subscribe();
    for i in 0..257 {
        stream
            .send(Event::AgentMessage {
                content: i.to_string(),
            })
            .unwrap();
    }
    assert!(matches!(
        receiver.recv().await,
        Err(tokio::sync::broadcast::error::RecvError::Lagged(1))
    ));
}
