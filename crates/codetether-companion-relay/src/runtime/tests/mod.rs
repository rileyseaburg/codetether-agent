//! Runtime behavior contracts exercised before and after module moves.
mod expiry;
mod fixture;
mod handler_fixture;
mod handlers;
mod limits;
mod queue;

use crate::runtime::Runtime;
use codetether_companion_protocol::SessionInput;

#[test]
fn initialization_preserves_input_and_starts_idle() {
    let runtime = Runtime::new(SessionInput {
        model: "test/vision".into(),
        prompt: "Describe".into(),
        interval_seconds: 30,
    });
    assert_eq!(runtime.model, "test/vision");
    assert_eq!(runtime.prompt, "Describe");
    assert_eq!(runtime.interval, 30);
    assert_eq!(runtime.status, "waiting");
    assert!(runtime.text.is_empty() && runtime.previous.is_empty());
    assert_eq!(
        (
            runtime.seq,
            runtime.frames,
            runtime.last_at,
            runtime.generation
        ),
        (0, 0, 0, 0)
    );
    assert!(!runtime.stopped);
    assert!(runtime.pending.is_none() && runtime.reply.is_none());
    assert!(runtime.active.is_none() && runtime.captured_at.is_none());
    assert!(runtime.viewers.is_empty());
}
