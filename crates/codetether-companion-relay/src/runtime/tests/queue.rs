//! Queue admission and acknowledgement outcomes.
use super::fixture::runtime;
use codetether_companion_protocol::TypedAck;
use serde_json::json;
#[test]
fn queue_rejects_unpaired_paused_stopped_and_busy_states() {
    for (paired, status, stopped) in [
        (false, "waiting", false),
        (true, "paused", false),
        (true, "waiting", true),
    ] {
        let mut state = runtime();
        state.status = status.into();
        state.stopped = stopped;
        assert_eq!(
            state
                .queue_reply(&json!({"text":"hello"}), paired, 100)
                .err()
                .unwrap()
                .status,
            409
        );
        assert!(state.reply.is_none());
    }
    let mut state = runtime();
    let receipt = state
        .queue_reply(&json!({"text":"  hello  "}), true, 100)
        .unwrap();
    assert_eq!(
        state
            .queue_reply(&json!({"text":"again"}), true, 101)
            .err()
            .unwrap()
            .status,
        409
    );
    assert!(!state.ack_reply(&TypedAck {
        reply_id: "unknown".into()
    }));
    assert_eq!(state.reply.as_ref().unwrap().text, "  hello  ");
    assert_eq!(state.seq, 0);
    assert!(state.ack_reply(&TypedAck {
        reply_id: receipt.reply_id.clone()
    }));
    assert_eq!(state.seq, 1);
    assert!(state.reply.is_none());
    assert!(!state.ack_reply(&TypedAck {
        reply_id: receipt.reply_id
    }));
    assert_eq!(state.seq, 1);
}
