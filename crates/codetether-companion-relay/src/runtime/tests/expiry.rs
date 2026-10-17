//! Expiration occurs at sixty seconds, even before validating new input.
use super::fixture::runtime;
use serde_json::{Value, json};

#[test]
fn expiry_publishes_one_error_at_the_boundary() {
    let mut state = runtime();
    let (sender, mut receiver) = tokio::sync::mpsc::channel(4);
    state.viewers.push(sender);
    state
        .queue_reply(&json!({"text":"hello"}), true, 100)
        .unwrap();
    assert!(state.command(60_099).reply.is_some());
    assert_eq!(state.seq, 0);
    assert!(state.command(60_100).reply.is_none());
    assert_eq!(state.seq, 1);
    let line = receiver.try_recv().unwrap();
    let event: Value = serde_json::from_str(line.strip_prefix("data: ").unwrap().trim()).unwrap();
    assert_eq!(
        event["text"],
        "Windows did not type the reply within 60 seconds."
    );
    state.expire_reply(120_100);
    assert_eq!(state.seq, 1);
    assert!(receiver.try_recv().is_err());
}

#[test]
fn invalid_new_reply_still_expires_the_old_reply() {
    let mut state = runtime();
    state
        .queue_reply(&json!({"text":"hello"}), true, 100)
        .unwrap();
    assert_eq!(
        state
            .queue_reply(&json!({"text":""}), true, 60_100)
            .err()
            .unwrap()
            .status,
        400
    );
    assert!(state.reply.is_none());
    assert_eq!(state.seq, 1);
}
